mod cli;
use structopt::StructOpt;
mod tasks;

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
