use crate::serenity::client::Context;
use crate::serenity::model::channel::Message;
use crate::serenity::framework::standard::{CommandResult, macros::command};
use scraper::{Html, Selector};
use reqwest;

pub const POEM_GROUP: &str = "Poem";

#[command]
pub async fn on_poem_command(ctx: &Context, msg: &Message) -> CommandResult {
    let response = reqwest::blocking::get("https://www.poetryfoundation.org/poems/poem-of-the-day")
        .unwrap_or_else(|e| panic!("Failed to send request: {}", e))
        .text()
        .unwrap_or_else(|e| panic!("Failed to get response text: {}", e));
        
    let document = Html::parse_document(&response);

    let title_selector = Selector::parse("h1.c-hdgSerif.c-hdgSerif_1").unwrap();
    let content_selector = Selector::parse(".o-poem").unwrap();

    let content: Vec<_> = document
        .select(&content_selector)
        .flat_map(|element| element.text())
        .collect();

    if !content.is_empty() {
        let poem_content = content.join("\n");
        msg.channel_id.say(&ctx.http, poem_content).await?;
    } else {
        msg.channel_id.say(&ctx.http, "No poem content found").await?;
    }

    Ok(())
}
