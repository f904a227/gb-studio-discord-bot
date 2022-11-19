mod role_buttons;

pub(super) mod prelude {
    pub(super) use super::{ButtonCreate, ButtonRespond};
    pub(super) use serenity::{
        builder::{CreateButton, CreateInteractionResponse},
        model::application::{
            component::ButtonStyle, interaction::message_component::MessageComponentInteraction,
        },
    };
}

pub(crate) use role_buttons::{
    ArtistRoleButton, BetaTesterRoleRoleButton, DesignerRoleButton, HardwareEnthusiastRoleButton,
    MusicianRoleButton, ScripterRoleButton,
};
use serenity::{
    builder::{CreateButton, CreateInteractionResponse},
    model::application::interaction::message_component::MessageComponentInteraction,
};

pub(crate) trait ButtonCreate {
    const CUSTOM_ID: &'static str;

    fn create(button: &mut CreateButton) -> &mut CreateButton;
}

pub(crate) trait ButtonRespond: ButtonCreate {
    fn respond<'a, 'b>(
        component: &MessageComponentInteraction,
        response: &'b mut CreateInteractionResponse<'a>,
    ) -> &'b mut CreateInteractionResponse<'a>;
}
