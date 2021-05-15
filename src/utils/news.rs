use crate::utils::article::Article;
use std::io;

pub struct News {
    pub articles: Vec<Article>,
}

impl News {
    pub fn from_articles(articles: Vec<Article>) -> News {
        News { articles }
    }

    pub fn print_for_alfred(&self) -> Result<(), Box<dyn std::error::Error>> {
        let items = self
            .articles
            .iter()
            .map(News::create_item)
            .collect::<Vec<alfred::Item<'_>>>();
        News::print_items(items)?;
        Ok(())
    }

    pub fn create_item(article: &Article) -> alfred::Item<'_> {
        return alfred::ItemBuilder::new(&article.title)
            .subtitle(&article.link)
            .quicklook_url(&article.link)
            .variable("url", &article.link)
            .into_item();
    }

    fn print_items(items: Vec<alfred::Item<'_>>) -> Result<(), Box<dyn std::error::Error>> {
        alfred::json::write_items(io::stdout(), &items)?;
        Ok(())
    }
}
