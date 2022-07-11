fn main() {
    let height_cm = input("身長(cm)は？");
    let weight = input("体重(kg)は？");
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
}
