// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/main.rs
// Entrypoint for Bot from Orbise


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

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Please set DISCORD_TOKEN");

    let intents = GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating Discord client");

    println!("Connecting to Discord...");

    if let Err(error) = client.start().await {
        eprintln!("Discord connection error: {error}");
    }
}

