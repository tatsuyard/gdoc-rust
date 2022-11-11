use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "fizzbuzz_file2_result.txt";
    let data = get_fizzbuzz(100);
}

fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();
    for i in 1..=max {
        if (i % 3 == 0) && (i % 5 == 0) {
            result += "FizzBuzz\n";
        }
    }
}
