use super::{get_fields, ToLuaData};
use mlua::UserData;
use poise::serenity_prelude::Role as DiscordRole;

pub(in super::super) struct Role(pub DiscordRole);
impl UserData for Role {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(fields, [id]);
    }
}

