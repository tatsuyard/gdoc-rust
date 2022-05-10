mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    let CommandLineArgs {} = CommandLineArgs::from_args();
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
