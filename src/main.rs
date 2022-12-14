mod commands;
mod components;
mod content;

use crate::{
    commands::{ContributeSlashCommand, DocsSlashCommand, PingSlashCommand, RolesSlashCommand},
    components::buttons::{
        ArtistRoleButton, BetaTesterRoleRoleButton, DesignerRoleButton,
        HardwareEnthusiastRoleButton, MusicianRoleButton, ScripterRoleButton,
    },
};
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{
        application::{command::Command, interaction::Interaction},
        gateway::{Activity, Ready},
        id::GuildId,
    },
    prelude::*,
};
use std::env;

lazy_static! {
    static ref DISCORD_BOT_TOKEN: String = {
        env::var("DISCORD_BOT_TOKEN")
            .expect("Failed to fetch the environment variable DISCORD_BOT_TOKEN")
    };
    static ref DISCORD_GUILD_ID: GuildId = {
        GuildId(
            env::var("DISCORD_GUILD_ID")
                .expect("Failed to fetch the environment variable DISCORD_GUILD_ID")
                .parse()
                .expect("Failed to parse the environment variable DISCORD_GUILD_ID"),
        )
    };
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        #[cfg(debug_assertions)]
        println!("Received interaction: {interaction:#?}");

        match interaction {
            Interaction::ApplicationCommand(command) => {
                slash_command_respond!(
                    ctx,
                    command,
                    [
                        ContributeSlashCommand,
                        DocsSlashCommand,
                        PingSlashCommand,
                        RolesSlashCommand
                    ]
                )
                .await
            }
            Interaction::Autocomplete(autocomplete) => {
                slash_command_autocomplete!(ctx, autocomplete, [DocsSlashCommand]).await
            }
            Interaction::MessageComponent(mut component) => {
                component_respond!(
                    ctx,
                    component,
                    [
                        ArtistRoleButton,
                        BetaTesterRoleRoleButton,
                        DesignerRoleButton,
                        HardwareEnthusiastRoleButton,
                        MusicianRoleButton,
                        ScripterRoleButton
                    ]
                )
                .await
            }
            unsupported_interaction => {
                unimplemented!("Unsupported interaction: {unsupported_interaction:?}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.tag());

        ctx.set_activity(Activity::playing(env!("CARGO_PKG_VERSION")))
            .await;

        GuildId::set_application_commands(&DISCORD_GUILD_ID, &ctx.http, |commands| {
            slash_command_register!(commands, [RolesSlashCommand])
        })
        .await
        .expect("Failed to set guild application commands");

        Command::set_global_application_commands(&ctx.http, |commands| {
            slash_command_register!(
                commands,
                [ContributeSlashCommand, DocsSlashCommand, PingSlashCommand]
            )
        })
        .await
        .expect("Failed to set global application commands");

        println!("Ready!");
    }
}

#[tokio::main]
async fn main() {
    lazy_static::initialize(&DISCORD_BOT_TOKEN);
    lazy_static::initialize(&DISCORD_GUILD_ID);

    let mut client = Client::builder(&*DISCORD_BOT_TOKEN, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Failed to create the client");

    if let Err(err) = client.start().await {
        eprintln!("Client error: {err:?}");
    }
}
