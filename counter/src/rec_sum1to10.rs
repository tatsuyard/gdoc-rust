fn sum(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    return sum(n - 1) + n;
}

fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}

fn main() {
    println!("{}", sum(10));
    for i in 2..=20 {
        println!("{}", fib(i));
    }
}
