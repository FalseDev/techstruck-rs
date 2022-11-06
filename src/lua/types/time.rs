use mlua::UserData;
use poise::serenity_prelude::Timestamp as DiscordTimestamp;

use super::{add_methods, ToLuaData};

pub(in super::super) struct Timestamp(pub DiscordTimestamp);
impl UserData for Timestamp {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_methods!(
            methods,
            [unix_timestamp],
            // [date],
            // [format],
            // [format_with_items],
            // [naive_local],
            // [naive_utc],
            // [offset],
            // [time],
            // [timestamp],
            [timestamp_millis],
            [timestamp_nanos],
            [timestamp_subsec_micros],
            [timestamp_subsec_millis],
            [timestamp_subsec_nanos],
            // [timezone],
            [to_rfc2822],
            [to_rfc3339],
            // [to_rfc3339_opts],
            // [with_timezone],
        );
    }
}
