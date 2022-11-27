use super::prelude::*;

pub(crate) struct ContributeSlashCommand;

impl SlashCommandRegister for ContributeSlashCommand {
    const NAME: &'static str = "contribute";

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(Self::NAME)
            .description("Sends information about contributing to the bot")
    }
}

#[async_trait]
impl SlashCommandRespond for ContributeSlashCommand {
    async fn respond(ctx: Context, interaction: &ApplicationCommandInteraction) {
        if let Err(err) = interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.content("You can contribute to this bot on GitHub: https://github.com/f904a227/gb-studio-discord-bot.")
                        .flags(MessageFlags::EPHEMERAL | MessageFlags::SUPPRESS_EMBEDS)
                    })
            })
            .await
        {
            eprintln!("Failed to create an interaction response: {err}");
        }
    }
}
