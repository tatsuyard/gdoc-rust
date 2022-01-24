use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add { task: String },
    Done { position: usize },
    List,
}

pub struct CommandLineArgs {
    pub action: Action,
    pub journal_file: Option<PathBuf>,
}
