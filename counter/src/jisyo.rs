use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dicfile = "";

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] jisyo word");
        return;
    }
}
