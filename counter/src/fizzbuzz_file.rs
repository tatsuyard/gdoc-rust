use std::fs::{self, File};
use std::io::{BufWriter, Write};

fn main() {
    let filename = "fizzbuzz_file_result.txt";
    {
        let fp = file::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        for i in 1..=100 {
            let mut line = format!("{}\n", i);
        }
    }
}
