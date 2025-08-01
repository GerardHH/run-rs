use anyhow::{bail, Result};
use clap::Parser;

mod action;
mod util;

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

    let task_files = action::find::task_files()?;
    let tasks = action::parse::tasks(task_files)?;
    println!("{:#?}", tasks);

    if cli.rerun {
        bail!("WIP");
    }

    Ok(())
}
