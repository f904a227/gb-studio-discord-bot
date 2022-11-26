pub(self) mod prelude {
    pub(super) use super::{ComponentCreate, ComponentRespond};
    pub(super) use serenity::{
        async_trait, client::Context,
        model::application::interaction::message_component::MessageComponentInteraction,
    };
}
pub(crate) mod buttons;

use serenity::{
    async_trait, client::Context,
    model::application::interaction::message_component::MessageComponentInteraction,
};

pub(crate) trait ComponentCreate {
    const CUSTOM_ID: &'static str;

    type CreateBuilder;

    fn create(create_builder: &mut Self::CreateBuilder) -> &mut Self::CreateBuilder;
}

#[async_trait]
pub(crate) trait ComponentRespond: ComponentCreate {
    async fn respond(ctx: Context, component: &MessageComponentInteraction)
        -> serenity::Result<()>;
}

#[macro_export]
macro_rules! component_respond {
    ( $ctx:expr, $component:expr, [$( $component_respond:ident ),*] ) => {
        {
            use $crate::components::{ComponentCreate, ComponentRespond};

            match $component.data.custom_id.as_str() {
                $(
                    <$component_respond as ComponentCreate>::CUSTOM_ID => <$component_respond as ComponentRespond>::respond($ctx, &$component),
                )*
                component_id => {
                    unimplemented!("Unhandled component interaction {component_id}");
                }
            }
        }
    };
}
