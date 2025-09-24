use clap::{ArgAction, Parser};
use std::path::PathBuf;
use std::str::FromStr;

use talos::{
    model::{Document, ErrorEntry},
    scanner::{scan_project, ScanOptions},
    types::{Extensions, GlobPatterns, MaxFileSize, OutputPath},
    writer::write_output,
};

/// Talos: Extract concise code signatures from projects (JS/TS first).
#[derive(Parser, Debug)]
#[command(
    name = "talos",
    version,
    about = "Signature-focused code summarizer CLI"
)]
struct Args {
    /// Input directory path
    input: PathBuf,

    /// Output filename (use '-' for stdout). Defaults to 'talos.json' in input dir.
    #[arg(short, long)]
    output: Option<String>,

    /// Skip files with 0 signatures
    #[arg(long, action = ArgAction::SetTrue)]
    terse_output: bool,

    /// Comma-separated list of allowed extensions (overrides defaults).
    #[arg(long)]
    ext: Option<String>,

    /// Include glob (repeatable)
    #[arg(long)]
    include: Vec<String>,

    /// Exclude glob (repeatable)
    #[arg(long)]
    exclude: Vec<String>,

    /// Max file size in bytes (skip larger files)
    #[arg(long)]
    max_file_size: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let extensions = match &args.ext {
        Some(s) => Extensions::from_str(s).unwrap_or_else(|_| {
            eprintln!("Warning: Invalid extensions format, using defaults");
            Extensions::default()
        }),
        None => Extensions::default(),
    };

    let options = ScanOptions {
        allowed_exts: extensions,
        include_globs: GlobPatterns::from(args.include),
        exclude_globs: GlobPatterns::from(args.exclude),
        max_file_size: args.max_file_size.map(MaxFileSize::new),
        terse_output: args.terse_output,
    };

    let root = args.input;
    if !root.is_dir() {
        return Err("Error: input must be a directory".into());
    }

    let (mut doc, mut errors): (Document, Vec<ErrorEntry>) =
        scan_project(&root, &options).map_err(|e| format!("Failed to scan project: {e}"))?;

    if !errors.is_empty() {
        doc.errors.append(&mut errors);
    }

    let output_path = OutputPath::new(args.output.as_deref(), &root);

    write_output(&doc, &output_path).map_err(|e| format!("Failed to write output: {e}").into())
}
