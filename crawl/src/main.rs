use select::document::Document;

fn main() -> eyre::Result<()> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
    .text()?;

    // println!("body = {:?}", body);
    let doc = Document::from(body.as_str());
    use select::predicate::Name;
    use url::Url;
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        println!("{:?}", Url::parse(href));
    }
    Ok(())
}
