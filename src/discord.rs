// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/discord.rs
// Module of Discord API


use serenity::{
    all::{CommandOptionType, GatewayIntents, GuildId, Interaction, Ready, Message},
    async_trait,
    builder::{
        CreateCommand, CreateCommandOption,
        CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    prelude::*,
};
use tracing::{error, info, debug};

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

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!(user = %ready.user.name, "Bot connected");

        let guild_id = std::env::var("DISCORD_GUILD_ID")
            .expect("Please set DISCORD_GUILD_ID")
            .parse::<u64>()
            .expect("Invalid DISCORD_GUILD_ID");

        let command = CreateCommand::new("orbise")
            .description("Orbise Bot commands")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "hello",
                    "Replies with Hello World",
                ),
            );

        match GuildId::new(guild_id)
            .set_commands(&ctx.http, vec![command])
            .await
        {
            Ok(_) => info!("Slash command registered"),
            Err(err) => error!(?err, "Failed to register slash command"),
        }
    }

    async fn interaction_create(
        &self,
        ctx: Context,
        interaction: Interaction,
    ) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        if command.data.name != "orbise" {
            return;
        }

        let Some(subcommand) = command.data.options.first() else {
            return;
        };

        if subcommand.name != "hello" {
            return;
        }

        if let Err(err) = command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Hello World"),
                ),
            )
            .await
        {
            error!(?err, "Failed to respond to slash command");
        }
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

