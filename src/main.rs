// use std::collections::HashMap;
use feed_rs::{parser, model::{Entry, Feed}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed = get_feed("https://feeds.arstechnica.com/arstechnica/index").await.expect("Encountered an error getting feed");
    
    for entry in feed.entries {
        print_entry(&entry);
    }

    Ok(())
}

async fn get_feed(feed_url: &str) -> Result<Feed, reqwest::Error> {
    let resp = reqwest::get(feed_url)
        .await?
        .text()
        .await?;
    let feed = parser::parse(resp.as_bytes()).expect("Failed to parse feed");
    Ok(feed)
}

fn print_entry (entry: &Entry) {
    match entry.title.as_ref() {
        Some(title) => {
            print!("• {}", title.content);
        }
        None => ()
    }
    match entry.links.get(0) {
        Some(link) => {
            println!(" - {}", link.href)
        }
        None => println!("")
    }
}
