use std::fs;

fn main() {
    let afile = './fizzbuzz_python.txt';

    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();
}