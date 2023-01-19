struct Item(String, i64);
fn main() {
    let banana = ("バナナ", 300);
    let apple = ("リンゴ", 200);
    let total = banana.1 + apple.1;
    print_tuple(&banana);
    print_tuple(&apple);
    println!("合計{}円です", total);
}

fn print_tuple(item: &(&str, i64)) {
    println!("{}を{}円で購入", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
}
