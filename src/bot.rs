use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, CommandError, macros::command};

mod scrapper;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || !msg.content.starts_with("!poem") {
            return;
        }

        if let Err(err) = scraper_module::on_poem_command(&ctx, &msg).await {
            eprintln!("Error processing !poem command: {}", err);
        }

    }
}

pub async fn run_bot() {
    let token = "DISCORD_BOT_TOKEN";
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(
            serenity::framework::StandardFramework::new()
                .configure(|c| c.prefix("!"))
                .group(&scraper_module::POEM_GROUP),
        )
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}