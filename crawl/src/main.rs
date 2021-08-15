use crawl::LinkExtractor;
use reqwest::blocking::ClientBuilder;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "n")]
    maximum_pages: usize,
    start_page: Url,
}
fn main() -> eyre::Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    // let url = std::env::args()
    //     .nth(1)
    //     .unwrap_or("https://www.rust-lang.org".to_owned());

    let url = Url::parse(&url)?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let wait = Duration::from_mills(100);

    let links = extractor.get_links(url)?;
    for url in crawler.take(10) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
