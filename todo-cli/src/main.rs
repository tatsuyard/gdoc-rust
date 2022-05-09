mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
