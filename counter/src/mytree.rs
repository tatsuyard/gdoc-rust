use std::{env, path};
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    if args.len() >= 2 {
        target_dir = &args[1];
    }
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

fn tree(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("存在しないパス");
}
