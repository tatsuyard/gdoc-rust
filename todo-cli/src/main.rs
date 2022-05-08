mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Action::*, CommandLineArgs};

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
