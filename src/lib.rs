mod errors;
mod tui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use color_eyre::{eyre::WrapErr, Result};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io;

use reqwest::blocking::get;
use scraper::{Html, Selector};

const BREAKING: &str = "https://www.goal.com/en-in/news";
const TRANSFERS: &str = "https://www.goal.com/en-in/category/transfers/1/k94w8e1yy9ch14mllpf4srnks";

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    current: usize,
    clicked: bool,
}
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            current: 0,
            clicked: false,
        }
    }

    pub fn select_item(&mut self) {
        self.state.select(Some(self.current));
    }

    // pub fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}
#[derive(Clone)]
pub struct Newz {
    url: Option<String>,
    heading: Option<String>,
}

pub struct App {
    exit: bool,
    items: StatefulList<Newz>,
}

impl App {
    pub fn new(news: Vec<Newz>) -> App {
        
        App {
            exit: false,
            items: StatefulList::with_items(news),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('n') => {
                self.next();
                self.items.select_item()
            }
            KeyCode::Char('m') => {
                self.previous();
                self.items.select_item()
            }

            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn next(&mut self) {
        if self.items.current == 1 {

            self.items.current = 0;
        } else {
            self.items.current += 1;
        }
    }
    fn previous(&mut self) {
        if self.items.current == 0 {
            self.items.current = 14;
        } else {
            self.items.current -= 1;
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = f.size();
    
    let items: Vec<ListItem> = app
        .items
        .items
        .clone()
        .iter()
        .take(15)
        .filter_map(|newz| newz.heading.clone())
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .map(|(index, i)| {
            let text = format!("{}. {}", index + 1, i);
            ListItem::new(text).style(Style::default().fg(Color::Black).bg(Color::White))
        })       
        .collect();
    
    let items_list = List::new(items)
        .block(Block::default().borders(Borders::TOP).title("PlayList"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items_list, chunks, &mut app.items.state)
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

pub fn print_breaking() -> Result<()> {
    let news = scrape_news(BREAKING);

    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let app_result = App::new(news).run(&mut terminal);
    tui::restore()?;
    app_result

    // println!("Breaking News");
    // for (i, item) in news.iter().enumerate().take(15) {
    //     println!(
    //         "{}. {}",
    //         i + 1,
    //         item.heading.as_ref().unwrap_or(&String::new())
    //     );
    // }
}

// pub fn print_transfers() {
//     let news = scrape_news(TRANSFERS);

//     println!("Transfer News");
//     for (i, item) in news.iter().enumerate().take(15) {
//         println!(
//             "{}. {}",
//             i + 1,
//             item.heading.as_ref().unwrap_or(&String::new())
//         );
//     }
// }
