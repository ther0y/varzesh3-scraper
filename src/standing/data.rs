use serde::{Deserialize, Serialize};

const JSON_DATA: &str = r#"
{
    "leagues": [
        {
            "id": "917",
            "title": "لیگ برتر ایران"
        }, 
        {
            "id": "846",
            "title": "لیگ برتر انگلیس"
        },
        {
            "id": "834",
            "title": "بوندسلیگا آلمان"
        }, 
        {
            "id": "862",
            "title": "لالیگا اسپانیا"
        }, 
        {
            "id": "877",
            "title": "سری‌آ ایتالیا"
        }, 
        {
            "id": "843",
            "title": "لیگ یک فرانسه"
        },
        {
            "id": "886",
            "title": "لیگ برتر پرتغال"
        }, 
        {
            "id": "923",
            "title": "لیگ آزادگان ایران"
        }
    ]
}
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct StandingList {
    pub leagues: Vec<StandingListItem>,
}

impl StandingList {
    pub fn get_leagues() -> Result<Vec<StandingListItem>, Box<dyn std::error::Error>> {
        let list: StandingList = serde_json::from_str(JSON_DATA).expect("failed to parse");
        Ok(list.leagues)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandingListItem {
    pub id: String,
    pub title: String,
}
