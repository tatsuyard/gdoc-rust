fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    let pr = "窮鼠猫を噛む";
    for c in pr.chars() {
        print!("[{}]", c);
    }
    println!("\n文字数={}字", pr.chars().count());

    let pr_chars: Vec<char> = pr.chars().collect();
    for c in pr_chars() {
        print!("({})", c);
    }
    println!("\n文字数={}字", pr_chars.len());
}
