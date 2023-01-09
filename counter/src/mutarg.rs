fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}
fn greet(msg: &String) {
    println!("{}", msg);
}
fn main() {
    let mut v = 16;
    x2(&mut v);

    let msg = String::from("hoge");
    greet(&msg);
    println!("{}", v);
}
