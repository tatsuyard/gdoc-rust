use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total: f64 = 0.0;
    for (i, fname) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let text = fs::read_to_string(fname).unwrap();
        let lines = text.split('¥n');
    }
}
