use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }
    let target_dir = &args[1];
}
