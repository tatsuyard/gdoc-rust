use std::fs;

fn main() {
    let files = fs::read_dir(".").expect("不正なパス");
    for ent in files {
        let entry = ent.unwrap();
        let path = ent.path();
    }
}
