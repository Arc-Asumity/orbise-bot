// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/discord.rs
// Module of Discord API


use serenity::{
    all::{GatewayIntents, Message, Ready},
    async_trait,
    prelude::*,
};
use tracing::{debug, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        info!(
            "[{}] {}: {}",
            message.guild_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "DM".to_owned()),
            message.author.name,
            message.content
        );
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Logged in as {}", ready.user.name);
    }
}

pub async fn start() -> serenity::Result<()> {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Please set DISCORD_TOKEN");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await?;

    debug!("Connecting to Discord...");

    client.start().await
}

