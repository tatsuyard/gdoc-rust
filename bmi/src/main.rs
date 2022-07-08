fn main() {
    let height_cm = input("身長(cm)は？");
    let weight = input("体重(kg)は？");
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
}
