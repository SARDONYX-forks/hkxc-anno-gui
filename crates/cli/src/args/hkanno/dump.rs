//! Dump & Update hkaano

use std::path::PathBuf;

use serde_hkx_features::progress::ProgressHandler;
use serde_hkx_hkanno::Format;

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true)]
pub(crate) struct DumpArgs {
    /// Input HKX file or directory containing HKX files
    #[arg(short = 'i', long = "input")]
    input: PathBuf,

    /// Output path for annotation files. If input is a file: can be a file path (e.g. output.txt) or directory. If input is a directory: must be a directory. Defaults to hkxc-anno-annos folder.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

pub(crate) async fn dump(args: &DumpArgs) -> Result<(), crate::args::AnyError> {
    let output_base = resolve_output_base(&args.input, args.output.as_ref()).await?;

    if args.input.is_file() {
        dump_file(&args.input, &output_base, args.output.as_ref()).await?;
    } else if args.input.is_dir() {
        dump_dir(
            &args.input,
            &output_base,
            crate::args::progress_handler::CliProgressHandler::new(),
        )
        .await?;
    } else {
        return Err("Expected dump input file/dir. But got neither.".into());
    }

    Ok(())
}

async fn resolve_output_base(
    input: &PathBuf,
    output: Option<&PathBuf>,
) -> Result<PathBuf, crate::args::AnyError> {
    let base = match output {
        Some(p) => p.clone(),
        None => PathBuf::from("hkxc-anno-annos"),
    };

    // Invalid dir input + file output
    if input.is_dir() {
        if let Some(out) = output {
            if out.is_file() {
                return Err("If directory is specified as input, output must not be file.".into());
            }
        }
        tokio::fs::create_dir_all(&base).await?;
    }

    Ok(base)
}

async fn dump_file(
    input: &PathBuf,
    output_base: &PathBuf,
    output: Option<&PathBuf>,
) -> Result<(), crate::args::AnyError> {
    let anno = serde_hkx_hkanno::editor::read_hkanno(input).await?;
    let Some(stem) = input.file_stem() else {
        return Err(format!("{}: Need file name. But not found.", input.display()).into());
    };

    let output_path = match output {
        Some(p) if p.is_dir() => p.join(stem).with_extension("txt"),
        Some(p) => p.clone(),
        None => {
            tokio::fs::create_dir_all(output_base).await?;
            output_base.join(stem).with_extension("txt")
        }
    };

    tokio::fs::write(output_path, anno).await?;
    Ok(())
}

async fn dump_dir<P>(
    input_dir: &PathBuf,
    output_base: &PathBuf,
    mut progress_handler: P,
) -> Result<(), crate::args::AnyError>
where
    P: ProgressHandler + Send + Sync + Clone + 'static,
{
    tokio::fs::create_dir_all(output_base).await?;

    let mut rd = tokio::fs::read_dir(input_dir).await?;
    let mut handles = tokio::task::JoinSet::new();

    let mut targets = Vec::new();

    // collect targets first (spawn しないものはここで弾く)
    while let Some(entry) = rd.next_entry().await? {
        let path = entry.path();

        let Some(ext) = path.extension() else {
            tracing::info!(
                path = %path.display(),
                "Skipping file without extension"
            );
            continue;
        };

        if Format::from_extension(ext).is_err() {
            tracing::info!(
                path = %path.display(),
                ext = %ext.to_string_lossy(),
                "Skipping non-HKX file"
            );
            continue;
        }

        targets.push(path);
    }

    if targets.is_empty() {
        progress_handler.on_empty();
        return Ok(());
    }

    progress_handler.on_set_total(targets.len());

    for input in targets {
        let output_base = output_base.clone();

        let progress_handler = progress_handler.clone();
        handles.spawn(async move {
            let result = async {
                let anno = serde_hkx_hkanno::editor::read_hkanno(&input).await?;

                let stem = input
                    .file_stem()
                    .ok_or_else(|| format!("invalid file name: {}", input.display()))?;

                let out = output_base.join(stem).with_extension("txt");

                if let Some(parent) = out.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                tokio::fs::write(&out, anno).await?;
                progress_handler.inc(1);
                Ok::<_, crate::args::AnyError>(out)
            }
            .await;

            (input, result)
        });
    }

    while let Some(joined) = handles.join_next().await {
        match joined {
            Ok((input, Ok(out))) => {
                progress_handler.on_processing_path(&input);
                progress_handler.success_inc(1);

                tracing::info!(
                    input = %input.display(),
                    output = %out.display(),
                    "Dumped annotation"
                );
            }
            Ok((input, Err(err))) => {
                progress_handler.on_processing_path(&input);
                progress_handler.failure_inc(1);

                tracing::error!(
                    error = %err,
                    path = %input.display(),
                    "Failed to dump annotation"
                );
            }
            Err(err) => {
                progress_handler.failure_inc(1);
                tracing::error!(
                    error = %err,
                    "Join error in dump task"
                );
            }
        }

        progress_handler.inc(1);
    }

    progress_handler.on_finish();
    Ok(())
}
