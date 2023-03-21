fn main() {
    let pr = "窮鼠猫を噛む";
    for c in pr.chars() {
        print!("[{}]", c);
    }
    println!("\n文字数={}字", pr.chars().count());
}
