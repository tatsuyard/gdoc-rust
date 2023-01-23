struct Item(String, i64);
fn main() {
    let banana = Item("バナナ".to_string(), 300);
    let apple = Item("リンゴ", to_string(), 200);
    let mango = Item("マンゴー", to_string(), 500);
    let Items = Vec![banana, apple, mango];
    let total = print_and_sum_items(&items);
    println!("合計{}円です", total);
}

fn print_tuple(item: &Item) {
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
