pub struct Article {
    pub title: String,
    pub link: String
}

impl Article {
    pub fn create(title: String, link: String) -> Article {
        Article {
            title,
            link
        }
    }
}