use reqwest::blocking::Client;

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

pub fn get_links(&self, url: Url) -> Reault<Vec<Url>, eyre::Report> {
    let reponse = self.client.get(url)?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());
    let mut links = Vec::new();
}