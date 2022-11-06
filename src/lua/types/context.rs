use std::sync::Arc;

use mlua::UserData;

use crate::{lua::types::get_fields, Context as CommandContext};

use super::toluadata::ToLuaData;

#[derive(Clone)]
pub(in super::super) struct Context(
    poise::serenity_prelude::Message,
    poise::serenity_prelude::Context,
);

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Context").field(&self.0).finish()
    }
}

impl Context {
    pub(crate) fn new(ctx: CommandContext<'_>) -> Self {
        Self(
            match ctx {
                poise::Context::Application(_) => todo!(),
                poise::Context::Prefix(ctx) => ctx.msg.clone(),
            },
            ctx.discord().clone(),
        )
    }
}

impl UserData for Context {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(fields, [author; post = clone()],);
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("guild", |_lua, this, _args: ()| {
            Ok(this.0.guild(&this.1).to_user_data())
        });
        methods.add_async_method("say", |_lua, this, text: String| async move {
            this.0
                .reply(this.1, text)
                .await
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
                .to_user_data();
            Ok(())
        });
    }
}

impl ToLuaData for CommandContext<'_> {
    type UserType = Context;

    fn to_user_data(self) -> Self::UserType
    where
        <Self as ToLuaData>::UserType: UserData,
    {
        Context::new(self)
    }
}
