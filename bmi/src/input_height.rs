fn main() {
    let mut height;
    loop {
        println!("身長(cm)は?");
        height = input_f(0.0);
        if height > 0.0 {
            break;
        }
        println!("標準体重は{:.1}kgです", weight);
    }
}

fn input_str() -> String {}
