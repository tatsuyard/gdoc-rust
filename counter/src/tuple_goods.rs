struct Item(String, i64);
fn main() {
    let banana = ("バナナ".to_string(), 300);
    let apple = ("リンゴ", to_string(), 200);
    let mango = Item("マンゴー", to_string(), 500);
    println!("合計{}円です", total);
}

fn print_tuple(item: &(&str, i64)) {
    println!("{}を{}円で購入", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total
}
