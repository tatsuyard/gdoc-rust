fn main() {
    hex_dmup("abcdef");
}

fn hex_dmup(s: &str) {
    for (i, c) in s.bytes().enumerate() {
        if i % 16 == 0 {
            print!("{:08x|", i);
        }
    }
}
