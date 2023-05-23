// mod search_index;
use crate::SearchIndex;
use url::Url;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
// use tokio::spawn;

fn url_to_title( url: &str ) -> String {
    let mut title = String::new();

    for ch in url.chars() {
        if ch == '/' || ch == ':' || ch == '.' {
            if title.chars().last() != Some( '-' ) {
                title.push( '-' );
            }
        } else {
            title.push( ch );
        }
    }
    if title.chars().last() == Some( '-' ) {
        title.pop();
    }
    return title;
}

fn add_doc_to_index( html: &str, title: &str, index: &mut SearchIndex ) {
    let document = Document::from( html );

    // Extract text from specific HTML elements and add it to the search index
    let binding = document.find( Name( "p" ) )
        .map( |term| term.text() )
        .collect::< Vec<String> >();
    let terms: &[String] = &binding;
    println!( "Added {title} to the SearchIndex" );
    index.add_document( title.to_string(), terms );
}
pub fn crawl( client: &Client, url: &str, depth: usize, visited_urls: &mut HashSet<String>, index: &mut SearchIndex ) {
    // Base case: If depth is zero or URL has already been visited, exit the recursion
    if depth == 0 || visited_urls.contains( url ) {
        return;
    }

    visited_urls.insert( url.to_owned() );

    println!( "Crawling: {}", url );

    match client.get( url ).send() {
        Ok( response ) => {
            if response.status().is_success() {
                let body = response.text().unwrap();
                let body_str = body.as_str();
                
                add_doc_to_index( &body_str, &url_to_title( url ), index );

                
                let document = Document::from( body_str );
                

                for link in document.find( Name( "a" ) ) {
                    if let Some( href ) = link.attr( "href" ) {
                        if let Ok( link_url ) = Url::parse( href ) {
                            // If the parsed URL has a host, use it as an absolute URL
                            let absolute_url_str = if link_url.has_host() {
                                link_url.to_string()
                            } else {
                                // If the parsed URL doesn't have a host, join it with the base URL of the current page
                                let base_url = Url::parse( url ).unwrap();
                                base_url.join( &link_url.to_string() ).unwrap().to_string()
                            };

                            let absolute_url = Url::parse( &absolute_url_str ).unwrap();
                            let scheme = absolute_url.scheme();
                            if scheme == "http" || scheme == "https" { // Crawling only works on HTTP(S)
                                // Recursively crawl the next URL with depth - 1
                                crawl( client, &absolute_url_str, depth - 1, visited_urls, index );
                            }
                        }
                    }
                }
            }
        }
        Err( err ) => {
            // An error occurred while sending the HTTP request
            eprintln!( "Error crawling {}: {}", url, err );
        }
    }
}
