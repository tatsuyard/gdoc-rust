use select::document::Document;

fn main() -> eyre::Result<()> {
    let response = reqwest::blocking::get("https://www.rust-lang.org")?;
    let base_url = response.url().clone();
    let body = response.text()?;
    // println!("body = {:?}", body);
    let doc = Document::from(body.as_str());
    use select::predicate::Name;
    use url::Url;
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        use url::ParseError as UrlParseError;
        // println!("{:?}", Url::parse(href));
        match Url::parse(href) {
            Ok(url) => { println!("{}", url); },
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                println!("{}", url);
            },
            Err(e) => {},
        }
    }
    Ok(())
}
