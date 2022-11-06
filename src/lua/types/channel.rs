use super::{add_methods, get_fields, ToLuaData};
use mlua::UserData;
use poise::serenity_prelude::{
    Channel as DiscordChannel, ChannelCategory as DiscordChannelCategory,
    GuildChannel as DiscordGuildChannel, PrivateChannel as DiscordPrivateChannel,
};
pub(in super::super) struct Channel(pub DiscordChannel);
impl UserData for Channel {
    // fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
    //     get_fields!(fields, [id;this.0.id().0]);
    // }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_methods!(
            methods,
            [guild; pre = clone()],
            [category; pre = clone()],
            [private; pre = clone()],
            [is_nsfw],
            [position],
        );
        methods.add_method("channel", |lua, this, _args: ()| {
            Ok(match this.0.clone() {
                DiscordChannel::Category(ch) => lua.create_userdata(ch.to_user_data()),
                DiscordChannel::Private(ch) => lua.create_userdata(ch.to_user_data()),
                DiscordChannel::Guild(ch) => lua.create_userdata(ch.to_user_data()),
                _ => unimplemented!("Unknown channel type"),
            })
        });
    }
}

pub(in super::super) struct GuildChannel(pub DiscordGuildChannel);
impl UserData for GuildChannel {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(
            fields,
            [id],
            [name; post = clone()],
            [bitrate],
            [default_auto_archive_duration],
            [guild_id],
            // [kind],
            [last_message_id],
            [last_pin_timestamp],
            // [member],
            [member_count],
            [message_count],
            [nsfw],
            [parent_id],
            // [permission_overwrites],
            [position],
            [rate_limit_per_user],
            [rtc_region; post = clone()],
            // [thread_metadata],
            [topic; post = clone()],
            [user_limit],
            // [video_quality_mode],
        );
    }
}

pub(in super::super) struct ChannelCategory(pub DiscordChannelCategory);
impl UserData for ChannelCategory {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(fields, [id], [name; post = clone()]);
    }
}

pub(in super::super) struct PrivateChannel(pub DiscordPrivateChannel);
impl UserData for PrivateChannel {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        get_fields!(fields, [id]);
    }
}
