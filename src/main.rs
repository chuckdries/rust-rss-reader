use futures::future;

use feed_rs::{
    model::{Entry, Feed},
    parser,
};
use kdl::KdlDocument;

struct FeedMeta {
    url: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feeds_metas = get_feeds();

    let responses =
        future::join_all(feeds_metas.iter().map(|meta| get_feed(meta))).await;

    for response in responses {
        let (name, feed) = response.expect("Encountered error getting feed");
        println!("--- {} ---", name);
        for entry in feed.entries {
            print_entry(&entry);
        }
    }

    Ok(())
}

fn get_feeds() -> Vec<FeedMeta> {
    // TODO: read from file
    let config_str = r#"
feeds {
    ArsTechnica "https://feeds.arstechnica.com/arstechnica/index"
    "The Verge" "http://www.theverge.com/rss/index.xml"
    "Mother Jones" "http://feeds.feedburner.com/motherjones/feed"
}
"#;

    let doc: KdlDocument = config_str.parse().expect("failed to parse KDL");

    let mut feeds: Vec<FeedMeta> = Vec::new();

    for node in doc
        .get("feeds")
        .expect("feeds param missing")
        .children()
        .expect("feeds empty")
        .nodes()
    {
        let name = node.name().to_string().replace("\"", "");
        let url = node
            .entries()
            .last()
            .expect("feed missing url")
            .value()
            .to_string()
            .replace("\"", "");
        feeds.push(FeedMeta { name, url });
    }

    return feeds;
}

async fn get_feed(meta: &FeedMeta) -> Result<(&String, Feed), reqwest::Error> {
    let resp = reqwest::get(&meta.url).await?.text().await?;
    let feed = parser::parse(resp.as_bytes()).expect("Failed to parse feed");
    Ok((&meta.name, feed))
}

fn print_entry(entry: &Entry) {
    match entry.title.as_ref() {
        Some(title) => {
            print!("• {}", title.content);
        }
        None => (),
    }
    match entry.links.get(0) {
        Some(link) => {
            println!(" ({})", link.href)
        }
        None => println!(""),
    }
}
