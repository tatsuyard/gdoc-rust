mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {}

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();
    let journal_file = journal_file.expect("Failed to find journal file");
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action");
}
