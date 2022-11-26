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

#[async_trait]
impl SlashCommandRespond for PingSlashCommand {
    async fn respond(ctx: Context, interaction: &ApplicationCommandInteraction) {
        if let Err(err) = interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| data.content("Pong!"))
            })
            .await
        {
            eprintln!("Failed to create an interaction response: {err}");
        }
    }
}
