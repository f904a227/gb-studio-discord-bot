use super::prelude::*;

pub(crate) struct PingSlashCommand;

impl SlashCommandRegister for PingSlashCommand {
    const NAME: &'static str = "ping";

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(Self::NAME)
            .description("Tests the availability of the bot")
    }
}

impl SlashCommandRespond for PingSlashCommand {
    fn respond<'a, 'b>(
        _interaction: &ApplicationCommandInteraction,
        response: &'b mut CreateInteractionResponse<'a>,
    ) -> &'b mut CreateInteractionResponse<'a> {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| data.content("Pong!"))
    }
}
