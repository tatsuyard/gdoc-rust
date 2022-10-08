use std::fs;

fn main() {
    let afile = './fizzbuzz_python.txt';
    let astr = fs::read_to_string(afile).unwrap();
}