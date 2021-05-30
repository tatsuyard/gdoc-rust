use reqwest::blocking::Client;
use select::document::Document;
use url::Url;
use log;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum GetLinksError {
    #[error("Failed to send a request")]
    SendRequest(#[source] reqwest::Error),
    #[error("Failed to read the response body")]
    ResponseBody(#[source] reqwest::Error),
    #[error("Failed to make the link URL absolute")]
    AbsolutizeUrl(#[source] url::ParseError),
    #[error("Server returned an error")]
    ServerError(#[source] reqwest::Error),
}

pub struct LinkExtractor {
    client: Client,
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self {
            client: client,
        }
    }

    pub fn get_links(&self, url: Url) -> Result<Vec<Url>, eyre::Report> {
        log::info!("GET \"{}\"", url);
        let response = self.client.get(url).send()
        .map_err(|e| GetLinksError::SendRequest(e))?;
        let base_url = response.url().clone();
        let status = response.status();
        let body = response.text()?;
        let doc = Document::from(body.as_str());
        use select::predicate::Name;
        let mut links = Vec::new();
        log::info!("Retrieved {} \"{}\"", status, base_url);

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
}
