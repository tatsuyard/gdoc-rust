fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // wordの中身は、値5になる

    s.clear(); // Stringを空にする。つまり、""と等しくする  
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

