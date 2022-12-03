use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }
    let target_dir = &args[1];
    let keyword = &args[2];
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("存在しないパス");
}
