mod standing;
mod utils;

use standing::{fetch_standing, print_standings_list};
use std::env;
use utils::{fetch_latest_news, fetch_slider_items, search};

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

        3 => {
            let action = &args[1];
            let modifier = &args[2];

            match &action[..] {
                "standings" => match &modifier[..] {
                    "list" => {
                        print_standings_list()?;
                    }
                    league_id => {
                        fetch_standing(league_id.to_string()).await?;
                    }
                },
                _ => {}
            }
        }

        _ => {
            fetch_latest_news().await?;
        }
    }

    Ok(())
}
