use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::utils::{Article, News};
use eyre::Result;

use scraper::{Html, Selector};

pub async fn fetch_latest_news() -> Result<()> {
    let news = scrape_news("https://www.varzesh3.com/", "#widget3 li a").await?;
    //news.print_for_alfred()?;

    let mut handles = Vec::new();
    let chunks = news.articles.as_slice()[0..48].to_vec();
    let responses: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    for article in chunks {
        // Spawn a thread and fetch article details
        // Caution: Do Not DDOS Them!

        let rsps = responses.clone();

        let handle = thread::spawn(move || {
            let d = reqwest::blocking::get(&article.link)
                .expect("err")
                .text()
                .expect("dcsd");

            rsps.lock().unwrap().push(d);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", responses.clone().lock().unwrap().len());

    Ok(())
}

pub async fn search(query: String) -> Result<()> {
    let url = format!("https://www.varzesh3.com/Search?q={}", query);
    let news = scrape_news(&url, ".complete-search-result-wrapper ul li h3 a").await?;

    news.print_for_alfred()?;

    Ok(())
}

pub async fn fetch_slider_items() -> Result<()> {
    let selector = ".news-slider--text-container a";
    let news = scrape_news("https://www.varzesh3.com/", selector).await?;

    news.print_for_alfred()?;

    Ok(())
}

async fn scrape_news(url: &str, select_path: &str) -> Result<News> {
    let res = reqwest::get(url).await?.text().await?;

    let fragment = Html::parse_fragment(&res);
    let selector = Selector::parse(select_path).unwrap();

    let ul = fragment.select(&selector);

    let articles = ul
        .map(|elem| {
            let link = format!(
                "{}{}",
                "https://www.varzesh3.com",
                elem.value().attr("href").unwrap()
            );

            let title = elem.text().next().unwrap().to_string();

            Article::create(title, link)
        })
        .collect();

    Ok(News::from_articles(articles))
}

pub async fn fetch_article_image(url: &str) -> Result<String> {
    let res = reqwest::get(url)
        .await
        .expect("c")
        .text()
        .await
        .expect("sdcsd");

    Ok(res)
}
