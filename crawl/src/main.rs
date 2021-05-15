use reqwest::blocking::ClientBuilder;
use url::Url;
use:crawler::LinkExtractor;

fn main() -> eyre::Result<()> {
    let url = Url::parse("https://www.rust-lang.org")?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);
    let links = extractor.get_links(url)?;
    for links in links.iter() {
        println!("{}", link);
    }
    
    Ok(())
}
