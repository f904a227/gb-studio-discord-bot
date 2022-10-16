mod ping;

pub(self) mod prelude {
    pub(super) use super::{SlashCommandRegister, SlashCommandRespond};
    pub(super) use serenity::{
        builder::{CreateApplicationCommand, CreateInteractionResponse},
        model::application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    };
}

pub(crate) use ping::PingSlashCommand;
use serenity::{
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

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
