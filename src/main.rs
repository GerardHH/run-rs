use anyhow::{bail, Result};
use clap::Parser;

/// A task runner that understands VSCode tasks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Rerun a previously executed task
    #[arg(short, long)]
    rerun: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.rerun {
        bail!("WIP");
    }

    Ok(())
}
