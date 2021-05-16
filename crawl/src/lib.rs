use reqwest::blocking::Client;
use select::document::Document;
use url::Url;

pub struct LinkExtractor {
    client: Client,
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self {
            client: client,
        }
    }
}

pub fn get_links(&self, url: Url) -> Result<Vec<Url>, eyre::Report> {
    let response = self.client.get(url).send()?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());
    use select::predicate::Name;
    let mut links = Vec::new();
    
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
    Ok(links)
}