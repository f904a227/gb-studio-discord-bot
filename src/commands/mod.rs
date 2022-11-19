mod docs;
mod ping;
mod roles;

pub(self) mod prelude {
    pub(super) use super::{SlashCommandAutocomplete, SlashCommandRegister, SlashCommandRespond};
    pub(super) use serenity::{
        builder::{
            CreateApplicationCommand, CreateAutocompleteResponse, CreateInteractionResponse,
        },
        model::application::interaction::{
            application_command::ApplicationCommandInteraction,
            autocomplete::AutocompleteInteraction, InteractionResponseType,
        },
    };
}

use serenity::{
    builder::{CreateApplicationCommand, CreateAutocompleteResponse, CreateInteractionResponse},
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
    },
};
pub(crate) use {docs::DocsSlashCommand, ping::PingSlashCommand, roles::RolesSlashCommand};

pub(crate) trait SlashCommandRegister {
    const NAME: &'static str;

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

pub(crate) trait SlashCommandRespond: SlashCommandRegister {
    fn respond<'a, 'b>(
        interaction: &ApplicationCommandInteraction,
        response: &'b mut CreateInteractionResponse<'a>,
    ) -> &'b mut CreateInteractionResponse<'a>;
}

pub(crate) trait SlashCommandAutocomplete: SlashCommandRegister {
    fn autocomplete<'a>(
        interaction: &AutocompleteInteraction,
        autocomplete: &'a mut CreateAutocompleteResponse,
    ) -> &'a mut CreateAutocompleteResponse;
}

#[macro_export]
macro_rules! slash_command_register {
    ( $commands:expr, [$( $cmd_register:ident ),*] ) => {
        {
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
            let f = match $command.data.name.as_str() {
                $(
                    $cmd_respond::NAME => <$cmd_respond as SlashCommandRespond>::respond,
                )*
                command_name => {
                    unimplemented!("Unhandled slash command {command_name}");
                }
            };

            $command
                .create_interaction_response(&$ctx.http, |response| f(&$command, response))
                .await
        }
    };
}

#[macro_export]
macro_rules! slash_command_autocomplete {
    ( $ctx:expr, $autocomplete:expr, [$( $cmd_autocomplete:ident ),*] ) => {
        {
            let f = match $autocomplete.data.name.as_str() {
                $(
                    $cmd_autocomplete::NAME => <$cmd_autocomplete as SlashCommandAutocomplete>::autocomplete,
                )*
                command_name => {
                    unimplemented!("Unhandled autocomplete request for slash command {command_name}");
                }
            };

            $autocomplete
                .create_autocomplete_response(&$ctx.http, |response| f(&$autocomplete, response))
                .await
        }
    };
}
