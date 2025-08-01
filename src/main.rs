use anyhow::{bail, Result};
use clap::Parser;

mod action;
mod parsers;
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

    let parsers: Vec<Box<dyn parsers::parser_trait::Parser>> =
        vec![Box::new(parsers::vscode::VSCode)];

    let supported_extensions = action::parse::supported_extension(&parsers);
    let task_files = action::find::task_files(cli.work_dir, supported_extensions)?;
    let tasks = action::parse::tasks(task_files, &parsers)?;
    if cli.list {
        println!("{:#?}", tasks);
    }

    if cli.rerun {
        bail!("WIP");
    }

    Ok(())
}
