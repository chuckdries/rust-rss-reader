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