use std::fs::{self, File};
use std::io::{BufWriter, Write};

fn main() {
    let filename = "fizzbuzz_file_result.txt";
    {
        let fp = file::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        for i in 1..=100 {
            let mut line = format!("{}\n", i);
            if (i % 3 == 0) && (i % 5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            }
        }
    }
}
