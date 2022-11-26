mod docs;
mod ping;
mod roles;

pub(self) mod prelude {
    pub(super) use super::{SlashCommandAutocomplete, SlashCommandRegister, SlashCommandRespond};
    pub(super) use serenity::{
        async_trait,
        builder::CreateApplicationCommand,
        client::Context,
        model::{
            application::interaction::{
                application_command::ApplicationCommandInteraction,
                autocomplete::AutocompleteInteraction, InteractionResponseType,
            },
            permissions::Permissions,
        },
    };
}

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
    },
};
pub(crate) use {docs::DocsSlashCommand, ping::PingSlashCommand, roles::RolesSlashCommand};

pub(crate) trait SlashCommandRegister {
    const NAME: &'static str;

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

#[async_trait]
pub(crate) trait SlashCommandRespond: SlashCommandRegister {
    async fn respond(ctx: Context, interaction: &ApplicationCommandInteraction);
}

#[async_trait]
pub(crate) trait SlashCommandAutocomplete: SlashCommandRegister {
    async fn autocomplete(ctx: Context, interaction: &AutocompleteInteraction);
}

#[macro_export]
macro_rules! slash_command_register {
    ( $commands:expr, [$( $cmd_register:ident ),*] ) => {
        {
            use $crate::commands::SlashCommandRegister;

            $commands
                $(
                    .create_application_command(<$cmd_register as SlashCommandRegister>::register)
                )*
        }
    };
}

#[macro_export]
macro_rules! slash_command_respond {
    ( $ctx:expr, $command:expr, [$( $cmd_respond:ident ),*] ) => {
        {
            use $crate::commands::{SlashCommandRespond, SlashCommandRegister};

            match $command.data.name.as_str() {
                $(
                    $cmd_respond::NAME => <$cmd_respond as SlashCommandRespond>::respond($ctx, &$command),
                )*
                command_name => {
                    unimplemented!("Unhandled slash command {command_name}");
                }
            }
        }
    };
}

#[macro_export]
macro_rules! slash_command_autocomplete {
    ( $ctx:expr, $autocomplete:expr, [$( $cmd_autocomplete:ident ),*] ) => {
        {
            use $crate::commands::{SlashCommandAutocomplete, SlashCommandRegister};

            match $autocomplete.data.name.as_str() {
                $(
                    $cmd_autocomplete::NAME => <$cmd_autocomplete as SlashCommandAutocomplete>::autocomplete($ctx, &$autocomplete),
                )*
                command_name => {
                    unimplemented!("Unhandled autocomplete request for slash command {command_name}");
                }
            }
        }
    };
}
