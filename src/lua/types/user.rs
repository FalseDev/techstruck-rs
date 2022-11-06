use mlua::UserData;
use poise::serenity_prelude::User as DiscordUser;
use poise::serenity_prelude::Member as DiscordMember;

use super::{get_fields, ToLuaData};

pub(crate) struct User(pub DiscordUser);
impl UserData for User {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(
            fields,
            [id;post = 0],
            [name;post = clone()],
            [discriminator],
            [bot],
            [banner;post = clone()],
            [avatar;post = clone()],
            // [accent_colour],
            // [public_flags],
        );
    }
}

pub(crate) struct Member(pub DiscordMember);
impl UserData for Member {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(
            fields,
            [avatar;post = clone()],
            // [accent_colour],
            // [public_flags],
        );
    }
}

