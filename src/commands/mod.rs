mod docs;
mod ping;

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
pub(crate) use {docs::DocsSlashCommand, ping::PingSlashCommand};

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
