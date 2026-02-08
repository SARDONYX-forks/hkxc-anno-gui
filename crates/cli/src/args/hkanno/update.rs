use serde_hkx_features::fs::ReadExt as _;
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true)]
pub(crate) struct UpdateArgs {
    /// Annotation source: can be a file or directory. If a file: applies to single input or all files in input directory. If a directory: uses it as the annotation folder to find matching .txt files. If omitted: uses default hkxc-anno-annos folder.
    #[arg(short = 'a', long = "anno")]
    anno: Option<PathBuf>,

    /// Input HKX file or directory containing HKX files to update
    #[arg(short = 'i', long = "input")]
    input: PathBuf,

    /// Output path for updated HKX files. If omitted, overwrites the input files.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Output format: 'amd64' for 64-bit Skyrim SE/AE, 'win32' for 32-bit Skyrim LE
    #[arg(short = 'v', long = "format")]
    format: serde_hkx_features::OutFormat,
}

pub(crate) async fn update(args: &UpdateArgs) -> Result<(), crate::args::AnyError> {
    // Annotation file or dir
    let anno_base = match args.anno.as_ref() {
        Some(p) => p,
        None => Path::new("hkxc-anno-annos"),
    };

    let output = args.output.as_deref();
    if args.input.is_file() {
        update_file(&args.input, anno_base, output, args.format).await?;
    } else if args.input.is_dir() {
        update_dir(&args.input, anno_base, output, args.format).await?;
    }

    Ok(())
}

async fn update_file(
    input: &Path,
    anno_base: &Path,
    output: Option<&Path>,
    format: serde_hkx_features::OutFormat,
) -> Result<(), crate::args::AnyError> {
    let hkanno_str = load_hkanno_str(input, anno_base).await?;
    let mut bytes = input.read_bytes().await?;

    let updated = serde_hkx_hkanno::parse_hkanno_str(&hkanno_str)?
        .update_hkx_bytes(&mut bytes, format, input)?;

    let out = output.unwrap_or(input);
    tokio::fs::write(out, updated).await?;

    Ok(())
}

async fn update_dir(
    input_dir: &Path,
    anno_base: &Path,
    output: Option<&Path>,
    format: serde_hkx_features::OutFormat,
) -> Result<(), crate::args::AnyError> {
    let mut rd = tokio::fs::read_dir(input_dir).await?;

    while let Some(entry) = rd.next_entry().await? {
        let input = entry.path();
        if input.extension().and_then(|s| s.to_str()) != Some("hkx") {
            continue;
        }

        let hkanno_str = load_hkanno_str(&input, anno_base).await?;
        let mut bytes = input.read_bytes().await?;

        let updated = serde_hkx_hkanno::parse_hkanno_str(&hkanno_str)?
            .update_hkx_bytes(&mut bytes, format, &input)?;

        let out = match output {
            Some(dir) => dir.join(input.file_name().unwrap()),
            None => input.clone(),
        };

        tokio::fs::write(out, updated).await?;
    }

    Ok(())
}

async fn load_hkanno_str(input: &Path, anno_base: &Path) -> Result<String, crate::args::AnyError> {
    if anno_base.is_file() {
        return Ok(anno_base.read_any_string().await?);
    }

    let stem = input
        .file_stem()
        .ok_or_else(|| format!("invalid input file: {}", input.display()))?;

    let anno_path = anno_base.join(stem).with_extension("txt");
    Ok(anno_path.read_any_string().await?)
}
