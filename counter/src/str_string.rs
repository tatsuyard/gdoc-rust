fn main() {
    let pr = "猫に小判";
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
}
