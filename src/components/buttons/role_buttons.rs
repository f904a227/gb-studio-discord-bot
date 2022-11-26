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
    async fn respond(
        ctx: Context,
        component: &mut MessageComponentInteraction,
    ) -> serenity::Result<()> {
        let member = if let Some(member) = component.member.as_mut() {
            member
        } else {
            return component
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content("**Error**: This button should only be used in a server!")
                        })
                })
                .await;
        };

        let guild_id = component.guild_id.expect("`member` data should be present");
        let roles = guild_id.roles(&ctx.http).await?;
        let role = roles.values().find(|role| role.name == R::NAME);

        let response_content = match role {
            Some(role_to_add) if !member.roles.contains(&role_to_add.id) => {
                member.add_role(&ctx.http, &role_to_add.id).await?;
                format!("**Success**: Added role {}.", role_to_add.name)
            }
            Some(role_to_remove) => {
                member.remove_role(&ctx.http, &role_to_remove.id).await?;
                format!("**Success**: Removed role {}.", role_to_remove.name)
            }
            None => {
                format!("**Error**: Missing role `{}` on the server!", R::NAME)
            }
        };

        component
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.content(response_content)
                            .flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await
    }
}

pub(crate) type ArtistRoleButton = RoleButton<ArtistRole>;
pub(crate) type BetaTesterRoleRoleButton = RoleButton<BetaTesterRole>;
pub(crate) type DesignerRoleButton = RoleButton<DesignerRole>;
pub(crate) type HardwareEnthusiastRoleButton = RoleButton<HardwareEnthusiastRole>;
pub(crate) type MusicianRoleButton = RoleButton<MusicianRole>;
pub(crate) type ScripterRoleButton = RoleButton<ScripterRole>;
