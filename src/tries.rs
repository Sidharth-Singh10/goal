use reqwest::blocking::get;
use scraper::{Html, Selector};

const LALIGA: &str = "https://www.skysports.com/la-liga-table";

struct LeagueTable {
    pos: String,
    team: String,
    played: String,
    won: String,
    drawn: String,
    lost: String,
    goalfor: String,
    goalagainst: String,
    goaldiff: String,
    points: String,
}

fn get_table(url: &str) -> Vec<LeagueTable> {
    let response = get(url).unwrap();
    let html_content = response.text().unwrap();
    let document = Html::parse_document(&html_content);
    let html_product_selector = Selector::parse("tr.standing-table__row").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut rows: Vec<LeagueTable> = Vec::new();

    for items in html_products {
        let mut row = LeagueTable {
            pos: String::new(),
            team: String::new(),
            played: String::new(),
            won: String::new(),
            drawn:String::new(),
            lost: String::new(),
            goalfor: String::new(),
            goalagainst: String::new(),
            goaldiff: String::new(),
            points: String::new(),
        };

        let data_selector = Selector::parse("td").unwrap();
        let mut i = 0;

        for td in items.select(&data_selector) {
            let text = td.text().collect::<String>();

            match i {
                0 => row.pos = text,
                1 => row.team = text,
                2 => row.played = text,
                3 => row.won = text,
                4 => row.drawn = text,
                5 => row.lost = text,
                6 => row.goalfor = text,
                7 => row.goalagainst = text,
                8 => row.goaldiff = text,
                9 => row.points = text,
                _ => {}
            }

            i += 1;
        }

        rows.push(row);
    }

    rows
}

pub fn print_table()
{
    let rows = get_table("https://www.skysports.com/la-liga-table");
    for (i, item) in rows.iter().enumerate().take(10) {
        println!("{}", item.points);
    }
}