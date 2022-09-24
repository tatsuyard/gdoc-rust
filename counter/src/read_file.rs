use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("入力ファイルを指定してください。");
        return;
    }
    let filename = &args[1];
}
