fn main() {
    let mut height;
    loop {
        println!("身長(cm)は?");
        height = input_f(0.0);
        if height > 0.0 {
            break;
        }
    }
}
