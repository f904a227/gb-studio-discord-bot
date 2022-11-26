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
        _ctx: Context,
        _component: &MessageComponentInteraction,
    ) -> serenity::Result<()> {
        todo!()
    }
}

pub(crate) type ArtistRoleButton = RoleButton<ArtistRole>;
pub(crate) type BetaTesterRoleRoleButton = RoleButton<BetaTesterRole>;
pub(crate) type DesignerRoleButton = RoleButton<DesignerRole>;
pub(crate) type HardwareEnthusiastRoleButton = RoleButton<HardwareEnthusiastRole>;
pub(crate) type MusicianRoleButton = RoleButton<MusicianRole>;
pub(crate) type ScripterRoleButton = RoleButton<ScripterRole>;
