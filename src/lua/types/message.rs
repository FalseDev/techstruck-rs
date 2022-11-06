use super::{get_fields, ToLuaData};
use mlua::UserData;
use poise::serenity_prelude::Message as DiscordMessage;

pub(in super::super) struct Message(pub DiscordMessage);
impl UserData for Message {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(fields, [id]);
    }
}
