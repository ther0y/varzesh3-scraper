use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableResult {
    #[serde(rename = "Sport")]
    pub sport: String,
    #[serde(rename = "Widget")]
    pub widget: Widget,
    #[serde(rename = "Table")]
    pub table: Vec<Team>,
    #[serde(rename = "Fixtures")]
    pub fixtures: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Widget {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "WidgetId")]
    pub widget_id: i64,
    #[serde(rename = "MatchGroupId")]
    pub match_group_id: i64,
    #[serde(rename = "StageId")]
    pub stage_id: i64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "GroupTitle")]
    pub group_title: String,
    #[serde(rename = "ShowTable")]
    pub show_table: bool,
    #[serde(rename = "ShowFixtures")]
    pub show_fixtures: bool,
    #[serde(rename = "WeekSelector")]
    pub week_selector: bool,
    #[serde(rename = "TableColor")]
    pub table_color: String,
    #[serde(rename = "FixturesColor")]
    pub fixtures_color: String,
    #[serde(rename = "StageFullName")]
    pub stage_full_name: String,
    #[serde(rename = "Slug")]
    pub slug: String,
    #[serde(rename = "Type")]
    pub type_field: i64,
    #[serde(rename = "Default")]
    pub default: bool,
    #[serde(rename = "MatchGroupSlug")]
    pub match_group_slug: String,
    #[serde(rename = "StandingsUrl")]
    pub standings_url: String,
    #[serde(rename = "ShowLink")]
    pub show_link: bool,
    #[serde(rename = "CustomLink")]
    pub custom_link: ::serde_json::Value,
    #[serde(rename = "CustomLinkTitle")]
    pub custom_link_title: ::serde_json::Value,
    #[serde(rename = "CustomLinkHighlight")]
    pub custom_link_highlight: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Team")]
    pub team: String,
    #[serde(rename = "Played")]
    pub played: i64,
    #[serde(rename = "Victories")]
    pub victories: i64,
    #[serde(rename = "Draws")]
    pub draws: i64,
    #[serde(rename = "Defeats")]
    pub defeats: i64,
    #[serde(rename = "Made")]
    pub made: i64,
    #[serde(rename = "Let")]
    pub let_field: i64,
    #[serde(rename = "Diff")]
    pub diff: i64,
    #[serde(rename = "Points")]
    pub points: i64,
    #[serde(rename = "PointsDeducted")]
    pub points_deducted: i64,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "IsNational")]
    pub is_national: bool,
    #[serde(rename = "CountryId")]
    pub country_id: i64,
    #[serde(rename = "Flag")]
    pub flag: Option<String>,
    #[serde(rename = "FlagHQ")]
    pub flag_hq: Option<String>,
    #[serde(rename = "TeamUrl")]
    pub team_url: Option<String>,
    #[serde(rename = "TeamHasDetailPage")]
    pub team_has_detail_page: bool,
}

impl Team {
    pub fn get_url(&self) -> String {
        match &self.team_url {
            Some(team_url) => {
                return format!("https://varzesh3.com{}", team_url);
            }
            None => {
                return "https://varzesh3.com".to_string();
            }
        }
    }
}
