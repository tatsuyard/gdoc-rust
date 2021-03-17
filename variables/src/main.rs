fn main() {
    let x = plus_one(5);
    println!("The value of f is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
