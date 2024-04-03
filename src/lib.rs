use reqwest::blocking::get;
use scraper::{Html, Selector};

const BREAKING: &str = "https://www.goal.com/en-in/news";
const TRANSFERS: &str = "https://www.goal.com/en-in/category/transfers/1/k94w8e1yy9ch14mllpf4srnks";

pub struct Newz {
    url: Option<String>,
    heading: Option<String>,
}

fn scrape_news(url: &str) -> Vec<Newz> {
    let response = get(url).unwrap();
    let html_content = response.text().unwrap();
    let document = Html::parse_document(&html_content);
    let html_product_selector = Selector::parse("li.item").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut store: Vec<Newz> = Vec::new();

    for html_product in html_products {
        let url = html_product
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        let heading = html_product
            .select(&Selector::parse("h3").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());

        store.push(Newz { url, heading });
    }

    store
}

pub fn print_breaking() {
    let news = scrape_news(BREAKING);

    println!("Breaking News");
    for (i, item) in news.iter().enumerate().take(10) {
        println!("{}. {}", i + 1, item.heading.as_ref().unwrap_or(&String::new()));
    }
}

pub fn print_transfers() {
    let news = scrape_news(TRANSFERS);

    println!("Transfer News");
    for (i, item) in news.iter().enumerate().take(10) {
        println!("{}. {}", i + 1, item.heading.as_ref().unwrap_or(&String::new()));
    }
}
