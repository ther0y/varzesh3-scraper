use crate::standing::result::{TableResult, Team};
use crate::standing::StandingList;
use eyre::Result;
use std::io;

pub async fn fetch_standing(standing_id: String) -> Result<()> {
    let url = format!(
        "https://api.varzesh3.com/v2.0/leaguestat/widget/5/{}",
        standing_id
    );

    let result = scrape_table(&url).await?;

    print_standings_table(result.0, result.1)?;

    Ok(())
}

pub fn print_standings_list() -> Result<()> {
    let leagues = StandingList::get_leagues()?;
    let items = leagues
        .iter()
        .map(|x| {
            alfred::ItemBuilder::new(x.title.to_string())
                .variable("league_id", x.id.to_string())
                .into_item()
        })
        .collect::<Vec<alfred::Item<'_>>>();

    alfred::json::write_items(io::stdout(), &items)?;

    Ok(())
}

pub fn print_standings_table(teams: Vec<Team>, standings_url: String) -> Result<()> {
    let mut position = 1;
    let items = teams
        .iter()
        .map(|x| {
            let title = format!("{}. {} - {} امتیاز", position, x.team, x.points);
            let team_url = x.get_url();
            let subtitle = format!(
                "MP:{}, W:{}, D:{}, L:{} \n {}",
                x.played, x.victories, x.draws, x.defeats, team_url
            );
            let link = format!("https://www.varzesh3.com{}", standings_url);
            position += 1;

            alfred::ItemBuilder::new(title)
                .subtitle(subtitle)
                .quicklook_url(link)
                .variable("team_url", team_url)
                .into_item()
        })
        .collect::<Vec<alfred::Item<'_>>>();

    alfred::json::write_items(io::stdout(), &items)?;

    Ok(())
}

async fn scrape_table(url: &str) -> Result<(Vec<Team>, String)> {
    let res = reqwest::get(url).await?.text().await?;

    let r: TableResult = serde_json::from_str(&res)?;
    Ok((r.table, r.widget.standings_url))
}
