// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/main.rs
// Entrypoint for Bot from Orbise

use tracing::error;

mod discord;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("debug,serenity=debug")
        .init();

    if let Err(error) = discord::start().await {
        error!("Discord connection error: {error}");
    }
}

