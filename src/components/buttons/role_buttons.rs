use crate::components::buttons::prelude::*;
use crate::content::roles::{
    ArtistRole, BetaTesterRole, DesignerRole, HardwareEnthusiastRole, MusicianRole, RoleDescribe,
    ScripterRole,
};
use std::marker::PhantomData;

pub(crate) struct RoleButton<R: RoleDescribe> {
    phantom: PhantomData<R>,
}

impl<R: RoleDescribe> ButtonCreate for RoleButton<R> {
    const CUSTOM_ID: &'static str = R::NAME;

    fn create(button: &mut CreateButton) -> &mut CreateButton {
        button
            .custom_id(Self::CUSTOM_ID)
            .emoji(R::EMOJI)
            .label(R::NAME)
            .style(ButtonStyle::Secondary)
    }
}

impl<R: RoleDescribe> ButtonRespond for RoleButton<R> {
    fn respond<'a, 'b>(
        _component: &MessageComponentInteraction,
        _response: &'b mut CreateInteractionResponse<'a>,
    ) -> &'b mut CreateInteractionResponse<'a> {
        todo!()
    }
}

pub(crate) type ArtistRoleButton = RoleButton<ArtistRole>;
pub(crate) type BetaTesterRoleRoleButton = RoleButton<BetaTesterRole>;
pub(crate) type DesignerRoleButton = RoleButton<DesignerRole>;
pub(crate) type HardwareEnthusiastRoleButton = RoleButton<HardwareEnthusiastRole>;
pub(crate) type MusicianRoleButton = RoleButton<MusicianRole>;
pub(crate) type ScripterRoleButton = RoleButton<ScripterRole>;
