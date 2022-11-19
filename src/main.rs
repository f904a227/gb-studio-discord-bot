mod commands;
mod components;
mod content;

use crate::{
    commands::{
        DocsSlashCommand, PingSlashCommand, RolesSlashCommand, SlashCommandAutocomplete,
        SlashCommandRegister, SlashCommandRespond,
    },
    components::buttons::{
        ArtistRoleButton, BetaTesterRoleRoleButton, ButtonCreate, ButtonRespond,
        DesignerRoleButton, HardwareEnthusiastRoleButton, MusicianRoleButton, ScripterRoleButton,
    },
};
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{application::interaction::Interaction, gateway::Ready, id::GuildId},
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

        let result = match interaction {
            Interaction::ApplicationCommand(command) => {
                slash_command_respond!(
                    ctx,
                    command,
                    [PingSlashCommand, DocsSlashCommand, RolesSlashCommand]
                )
            }
            Interaction::Autocomplete(autocomplete) => {
                slash_command_autocomplete!(ctx, autocomplete, [DocsSlashCommand])
            }
            Interaction::MessageComponent(component) => {
                let f = match component.data.custom_id.as_str() {
                    ArtistRoleButton::CUSTOM_ID => ArtistRoleButton::respond,
                    BetaTesterRoleRoleButton::CUSTOM_ID => BetaTesterRoleRoleButton::respond,
                    DesignerRoleButton::CUSTOM_ID => DesignerRoleButton::respond,
                    HardwareEnthusiastRoleButton::CUSTOM_ID => {
                        HardwareEnthusiastRoleButton::respond
                    }
                    MusicianRoleButton::CUSTOM_ID => MusicianRoleButton::respond,
                    ScripterRoleButton::CUSTOM_ID => ScripterRoleButton::respond,
                    component_id => {
                        unimplemented!("Unhandled component interaction {component_id}");
                    }
                };

                component
                    .create_interaction_response(&ctx.http, |response| f(&component, response))
                    .await
            }
            unsupported_interaction => {
                unimplemented!("Unsupported interaction: {unsupported_interaction:?}");
            }
        };

        if let Err(err) = result {
            eprintln!("Failed to respond to interaction: {err}");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.tag());

        let guild_commands =
            GuildId::set_application_commands(&DISCORD_GUILD_ID, &ctx.http, |commands| {
                slash_command_register!(
                    commands,
                    [PingSlashCommand, DocsSlashCommand, RolesSlashCommand]
                )
            })
            .await
            .expect("Failed to create guild application commands");

        println!("Created the following guild application commands: {guild_commands:#?}");

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
