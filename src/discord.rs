// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/discord.rs
// Module of Discord API

use serenity::{
    all::{GatewayIntents, Ready},
    async_trait,
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }
}

pub async fn start() -> serenity::Result<()> {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Please set DISCORD_TOKEN");

    let intents = GatewayIntents::GUILDS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await?;

    println!("Connecting to Discord...");

    client.start().await
}

