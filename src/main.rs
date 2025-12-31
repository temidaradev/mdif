use clap::Parser;
use filesize::PathExt;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Inspects file metadata")]
struct Args {
    path: PathBuf,

    #[arg(short, long)]
    human_readable: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let metadata = std::fs::metadata(&args.path)?;

    let size = if args.human_readable {
        format!("{} KB", metadata.len() / 1024)
    } else {
        format!("{} bytes", metadata.len())
    };

    println!("Path: {:?}\nSize: {}", args.path, size);
    Ok(())
}
