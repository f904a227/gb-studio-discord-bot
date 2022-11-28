use super::prelude::*;
use crate::{
    components::buttons::{
        ArtistRoleButton, BetaTesterRoleRoleButton, DesignerRoleButton,
        HardwareEnthusiastRoleButton, MusicianRoleButton, ScripterRoleButton,
    },
    {
        components::ComponentCreate,
        content::roles::{self, RoleDescribe},
    },
};
use serenity::model::mention::Mentionable;

pub(crate) struct RolesSlashCommand;

impl SlashCommandRegister for RolesSlashCommand {
    const NAME: &'static str = "roles";

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(Self::NAME)
            .description("Sends the roles menu in the current channel")
            .dm_permission(false)
            .default_member_permissions(Permissions::ADMINISTRATOR)
    }
}

#[async_trait]
impl SlashCommandRespond for RolesSlashCommand {
    async fn respond(ctx: Context, interaction: &ApplicationCommandInteraction) {
        fn describe_role_to_field<R: RoleDescribe>() -> (String, &'static str, bool) {
            (format!("{} {}", R::EMOJI, R::NAME), R::DESCRIPTION, true)
        }

        let fields = [
            describe_role_to_field::<roles::ArtistRole>(),
            describe_role_to_field::<roles::BetaTesterRole>(),
            describe_role_to_field::<roles::DesignerRole>(),
            describe_role_to_field::<roles::HardwareEnthusiastRole>(),
            describe_role_to_field::<roles::MusicianRole>(),
            describe_role_to_field::<roles::ScripterRole>(),
        ];

        let channel_id = interaction.channel_id;

        let result = channel_id
            .send_message(&ctx.http, |message| {
                message
                    .embed(|embed| embed.fields(fields))
                    .components(|components| {
                        components
                            .create_action_row(|action_row| {
                                action_row
                                    .create_button(ArtistRoleButton::create)
                                    .create_button(BetaTesterRoleRoleButton::create)
                                    .create_button(DesignerRoleButton::create)
                            })
                            .create_action_row(|action_row| {
                                action_row
                                    .create_button(HardwareEnthusiastRoleButton::create)
                                    .create_button(MusicianRoleButton::create)
                                    .create_button(ScripterRoleButton::create)
                            })
                    })
            })
            .await;

        let channel_mention = channel_id.mention();
        let content = match result {
            Ok(_) => format!("**Success**: Sent the role menu in {channel_mention}."),
            Err(err) => {
                eprintln!("Failed to send a message: {err:?}");
                format!("**Error**: Failed to send the role menu in {channel_mention}.")
            }
        };

        if let Err(err) = interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.content(content).flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await
        {
            eprintln!("Failed to create an interaction response: {err:?}");
        }
    }
}
