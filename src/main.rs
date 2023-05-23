mod crawl;
mod search_index;
use search_index::SearchIndex;
use crawl::crawl;
use reqwest::blocking::Client;
use std::collections::HashSet;
// use tokio::{ runtime::Builder, spawn };

fn main() {
    /*
    let runtime = Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
    */
    let start_urls = [ "https://github.com/", "https://wikipedia.org/", "https://nixos.org" ];
    let depth = 99999;

    let client = Client::new();
    let mut visited_urls = HashSet::new();
    let mut index = SearchIndex::new();

    for url in start_urls {
        crawl( &client, &url, depth, &mut visited_urls, &mut index );
    }
}
