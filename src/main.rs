// use std::io;
use scraper::{Html, Selector};

fn main() {
    let response = reqwest::blocking::get("https://www.poetryfoundation.org/poems/poem-of-the-day")
        .unwrap_or_else(|e| panic!("Failed to send request: {}", e))
        .text()
        .unwrap_or_else(|e| panic!("Failed to get response text: {}", e));
        
    let document = Html::parse_document(&response);

    let title_selector = Selector::parse("h1.c-hdgSerif.c-hdgSerif_1").unwrap();
    let content_selector = Selector::parse(".o-poem").unwrap();

    // process and print

    let titles: Vec<_> = document
        .select(&title_selector)
        .flat_map(|element| element.text())
        .collect();

    if !titles.is_empty() {
        println!("Title: {}", titles[0]);
    } else {
        println!("No title found");
    }

    let content: Vec<_> = document
        .select(&content_selector)
        .flat_map(|element| element.text())
        .collect();

    if !content.is_empty() {
        println!("Poem Content: {}", content.join("\n"));
    } else {
        println!("No poem content found");
    }
}
