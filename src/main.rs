use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::model::id::ChannelId;
use serenity::client::{Context, EventHandler};
use rand::seq::SliceRandom; // import for choose() function
use serenity::model::channel::ReactionType;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // ID of the channel you're interested in
        let target_channel_id = match env::var("DISCORD_TEST_CHANNEL_ID") {
            Ok(id) => match id.parse::<u64>() {
                Ok(id) => ChannelId(id),
                Err(_) => {
                    eprintln!("Failed to parse DISCORD_TEST_CHANNEL_ID as u64");
                    return;
                },
            },
            Err(_) => {
                eprintln!("Failed to read DISCORD_TEST_CHANNEL_ID from the environment");
                return;
            },
        };
        
        let zodiac_emojis = vec![
            "♈", // Aries
            "♉", // Taurus
            "♊", // Gemini
            "♋", // Cancer
            "♌", // Leo
            "♍", // Virgo
            "♎", // Libra
            "♏", // Scorpio
            "♐", // Sagittarius
            "♑", // Capricorn
            "♒", // Aquarius
            "♓", // Pisces
        ];

        if msg.channel_id == target_channel_id {
            // Choose a random emoji from the list
            let chosen_emoji = zodiac_emojis.choose(&mut rand::thread_rng());

            // If an emoji was successfully chosen (which should always be the case), react with it
            if let Some(emoji) = chosen_emoji {
                let _ = msg.react(&ctx.http, ReactionType::Unicode(emoji.to_string())).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}