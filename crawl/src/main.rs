use select::document::Document;

fn main() -> eyre::Result<()> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
    .text()?;

    // println!("body = {:?}", body);
    let doc = Document::from(body.as_str());
    use select::predicate::Name;
    for a in doc.find(Name("a")) {
        println!("{:?}", a);
    }
    Ok(())
}
