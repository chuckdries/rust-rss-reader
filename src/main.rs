// use std::collections::HashMap;
use feed_rs::{parser, model::Entry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://feeds.arstechnica.com/arstechnica/index")
        .await?
        .text()
        .await?;
    let feed = parser::parse(resp.as_bytes()).unwrap();
    
    for entry in feed.entries {
        print_entry(&entry);
    }

    Ok(())
}

fn print_entry (entry: &Entry) {
    let title = entry.title.as_ref().expect("missing title");
    println!("• {}", title.content)
}