// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/main.rs
// Entrypoint for Bot from Orbise

mod discord;

use serenity::{
    all::Ready,
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
    tracing_subscriber::fmt()
        .with_env_filter("serenity=trace")
        .init();

    if let Err(error) = discord::start().await {
        eprintln!("Discord connection error: {error}");
    }
}

