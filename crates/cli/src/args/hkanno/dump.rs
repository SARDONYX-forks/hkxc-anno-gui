//! Dump & Update hkaano

use std::path::PathBuf;

use serde_hkx_hkanno::OutFormat;

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
        dump_dir(&args.input, &output_base).await?;
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

async fn dump_dir(input_dir: &PathBuf, output_base: &PathBuf) -> Result<(), crate::args::AnyError> {
    let mut rd = tokio::fs::read_dir(input_dir).await?;

    while let Some(entry) = rd.next_entry().await? {
        let path = entry.path();

        if OutFormat::from_input(&path).is_err() {
            continue;
        }
        let anno = serde_hkx_hkanno::editor::read_hkanno(&path).await?;

        let out = {
            let input: &PathBuf = &path;
            let stem = input.file_stem().unwrap();
            output_base.join(stem).with_extension("txt")
        };
        tokio::fs::write(out, anno).await?;
    }

    Ok(())
}
