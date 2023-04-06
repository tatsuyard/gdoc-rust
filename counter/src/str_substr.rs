fn main() {
    let pr = "abcdefg";
    let mut sub1 = String::new();

    for (i, c) in pr.chars().enumerate() {
        if i < 2 {
            sub1.push(c);
            continuel;
        }
        break;
    }
    println!("先頭2文字: {}", sub1);
}
