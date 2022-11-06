use mlua::UserData;
use poise::serenity_prelude::Guild as DiscordGuild;

use super::get_fields;
use super::ToLuaData;

pub(in super::super) struct Guild(pub DiscordGuild);
impl UserData for Guild {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        // get_fields!(fields, [name clone]);
        get_fields!(
            fields,
            [id],
            [name;post=clone()],
            [description;post=clone()],
            [owner_id],
            [max_members],
            [afk_channel_id],
            [afk_timeout],
            [channels;post=clone()],
            [members;post=clone()],
            [roles;post=clone()],
            // [voice_states],
            // [emojis],
            [threads;post=clone()],
            // [stickers],
            [features;post=clone()],
            [icon;post=clone()],
            [splash;post=clone()],
            [discovery_splash;post=clone()],
            [system_channel_id],
            [system_channel_id],
            [rules_channel_id],
            [public_updates_channel_id],
            [premium_subscription_count],
            [banner;post=clone()],
            [vanity_url_code;post=clone()],
            [preferred_locale;post=clone()],
            // [joined_at],
            [large],
            [member_count],

        );
    }
}
