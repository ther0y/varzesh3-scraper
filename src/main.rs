mod utils;
use std::{env};
use utils::{search, fetch_latest_news,  fetch_slider_items};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            fetch_latest_news().await?;
        }

        2 => {
            let query = &args[1];

            match &query[..] {
                "top" => {
                    fetch_slider_items().await?;
                }
                "" => {
                    fetch_latest_news().await?;
                }
                _ => {
                    search(query.to_string()).await?;
                }
            }
        }
        _ => {
            fetch_latest_news().await?;
        }
    }

    Ok(())
}
