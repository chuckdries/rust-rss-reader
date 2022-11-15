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
    let feeds = get_feeds();

    for feed_meta in feeds {
        let feed = get_feed(feed_meta.url.as_str())
            .await
            .expect("Encountered an error getting feed");

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
        let name = node.name().to_string();
        let url = node.entries()[0].value().to_string().replace("\"", "");
        println!("{name} - {url}");
        feeds.push(FeedMeta { name, url });
    }

    return feeds;
}

async fn get_feed(feed_url: &str) -> Result<Feed, reqwest::Error> {
    let resp = reqwest::get(feed_url).await?.text().await?;
    let feed = parser::parse(resp.as_bytes()).expect("Failed to parse feed");
    Ok(feed)
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
            println!(" - {}", link.href)
        }
        None => println!(""),
    }
}
