use crawl::LinkExtractor;
use reqwest::blocking::ClientBuilder;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "n")]
    /// Maximum number of pages to be crawled
    maximum_pages: usize,
    start_page: Url,
}

fn main() -> eyre::Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let wait = Duration::from_mills(100);

    for url in crawler.take(opt.maximum_pages) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
