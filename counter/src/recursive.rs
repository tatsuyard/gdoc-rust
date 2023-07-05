fn recursive_function(n: u32) {
    if n == 0 {
        // ベースケース：再帰の終了条件
        return;
    }

    // 再帰呼び出し
    println!("現在の値: {}", n);
    recursive_function(n - 1);
}

fn main() {
    recursive_function(5);
}
