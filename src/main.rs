use anyhow::{bail, Result};
use clap::Parser;

mod action;
mod util;

/// A task runner that understands VSCode tasks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List all found tasks, ordered by task file paths
    #[arg(short, long)]
    list: bool,

    /// Rerun a previously executed task
    #[arg(short, long)]
    rerun: bool,

    /// The directory from where the tasks will be found.
    #[arg(short, long)]
    work_dir: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let task_files = action::find::task_files(cli.work_dir)?;
    if cli.list {
        println!("{:#?}", task_files);
    }

    if cli.rerun {
        bail!("WIP");
    }

    Ok(())
}
