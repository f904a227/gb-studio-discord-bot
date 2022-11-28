use crate::components::buttons::prelude::*;
use crate::content::roles::{
    ArtistRole, BetaTesterRole, DesignerRole, HardwareEnthusiastRole, MusicianRole, RoleDescribe,
    ScripterRole,
};
use std::marker::PhantomData;

pub(crate) struct RoleButton<R: RoleDescribe> {
    phantom: PhantomData<R>,
}

impl<R: RoleDescribe> ComponentCreate for RoleButton<R> {
    const CUSTOM_ID: &'static str = R::NAME;

    type CreateBuilder = CreateButton;

    fn create(button: &mut Self::CreateBuilder) -> &mut Self::CreateBuilder {
        button
            .custom_id(Self::CUSTOM_ID)
            .emoji(R::EMOJI)
            .label(R::NAME)
            .style(ButtonStyle::Secondary)
    }
}

#[async_trait]
impl<R: RoleDescribe> ComponentRespond for RoleButton<R> {
    async fn respond(ctx: Context, interaction: &mut MessageComponentInteraction) {
        let member = if let Some(member) = interaction.member.as_mut() {
            member
        } else {
            if let Err(err) = interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content("**Error**: This button should only be used in a server!")
                        })
                })
                .await
            {
                eprintln!("Failed to create an interaction response: {err:?}");
            }
            return;
        };

        let guild_id = interaction
            .guild_id
            .expect("`member` data should be present");

        let roles = match guild_id.roles(&ctx.http).await {
            Ok(roles) => roles,
            Err(err) => {
                eprintln!("Failed to fetch all roles of a guild: {err:?}");
                return;
            }
        };

        let role = roles.values().find(|role| role.name == R::NAME);

        let content = match role {
            Some(role_to_add) if !member.roles.contains(&role_to_add.id) => {
                if let Err(err) = member.add_role(&ctx.http, &role_to_add.id).await {
                    eprintln!("Failed to add a role to a member: {err:?}");
                    format!("**Error**: Failed to add role `{}`.", role_to_add.name)
                } else {
                    format!("**Success**: Added role `{}`.", role_to_add.name)
                }
            }
            Some(role_to_remove) => {
                if let Err(err) = member.remove_role(&ctx.http, &role_to_remove.id).await {
                    eprintln!("Failed to remove a role from a member: {err:?}");
                    format!(
                        "**Error**: Failed to remove role `{}`.",
                        role_to_remove.name
                    )
                } else {
                    format!("**Success**: Removed role `{}`.", role_to_remove.name)
                }
            }
            None => format!("**Error**: Missing role `{}` on the server!", R::NAME),
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

pub(crate) type ArtistRoleButton = RoleButton<ArtistRole>;
pub(crate) type BetaTesterRoleRoleButton = RoleButton<BetaTesterRole>;
pub(crate) type DesignerRoleButton = RoleButton<DesignerRole>;
pub(crate) type HardwareEnthusiastRoleButton = RoleButton<HardwareEnthusiastRole>;
pub(crate) type MusicianRoleButton = RoleButton<MusicianRole>;
pub(crate) type ScripterRoleButton = RoleButton<ScripterRole>;
