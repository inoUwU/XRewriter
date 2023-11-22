use dotenvy::dotenv;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

static DOMAIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(x|twitter)\.com").unwrap());
static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(https?://[^? ]+)(\?[^ ]+)?").unwrap());

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.is_own(&ctx.cache) && DOMAIN_REGEX.is_match(&msg.content) {
            let replaceed = DOMAIN_REGEX.replace_all(&msg.content, "fixupx.com");
            if let Some(cap) = URL_REGEX.captures(&replaceed) {
                if let Err(why) = msg.channel_id.say(&ctx.http, &cap[0]).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let discordtoken = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discordtoken, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
