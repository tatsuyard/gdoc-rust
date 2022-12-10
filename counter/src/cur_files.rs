use std::fs;

fn main() {
    let files = fs::read_dir(".").expect("不正なパス");
}
