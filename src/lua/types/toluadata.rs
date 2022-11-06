use std::{collections::HashMap, hash::Hash};

use mlua::ToLua;
use poise::serenity_prelude::{
    id::{
        AttachmentId, ChannelId, EmojiId, GuildId, MessageId, RoleId, StickerId, UserId, WebhookId,
    },
    Channel as DiscordChannel, ChannelCategory as DiscordChannelCategory, Guild as DiscordGuild,
    GuildChannel as DiscordGuildChannel, Member as DiscordMember, Message as DiscordMessage,
    PrivateChannel as DiscordPrivateChannel, Role as DiscordRole, Timestamp as DiscordTimestamp,
    User as DiscordUser,
};

use super::{
    Channel, ChannelCategory, Guild, GuildChannel, Member, Message, PrivateChannel, Role,
    Timestamp, User,
};

pub(in super::super) trait ToLuaData
where
    for<'a> Self::UserType: ToLua<'a>,
{
    type UserType;
    fn to_user_data(self) -> Self::UserType;
}

impl<T: ToLuaData> ToLuaData for Option<T> {
    type UserType = Option<<T as ToLuaData>::UserType>;

    fn to_user_data(self) -> Self::UserType {
        self.map(ToLuaData::to_user_data)
    }
}

impl<T> ToLuaData for Vec<T>
where
    T: ToLuaData,
{
    type UserType = Vec<<T as ToLuaData>::UserType>;

    fn to_user_data(self) -> Self::UserType {
        self.into_iter().map(ToLuaData::to_user_data).collect()
    }
}

impl<K, V> ToLuaData for HashMap<K, V>
where
    K: ToLuaData,
    V: ToLuaData,
    <K as ToLuaData>::UserType: Hash,
    <K as ToLuaData>::UserType: std::cmp::Eq,
    HashMap<<K as ToLuaData>::UserType, <V as ToLuaData>::UserType>: for<'a> ToLua<'a>,
{
    type UserType = HashMap<<K as ToLuaData>::UserType, <V as ToLuaData>::UserType>;

    fn to_user_data(self) -> Self::UserType {
        self.into_iter()
            .map(|(k, v)| (k.to_user_data(), v.to_user_data()))
            .collect()
    }
}

macro_rules! impl_toluadata {
    ($([$names:ty ; $user_type:ty $(; $wrap:expr)? $(=>$($op:tt)+)?]),+ $(,)?) => {
        $(
        impl ToLuaData for $names {
            type UserType = $user_type;
            #[inline(always)]
            fn to_user_data(self) -> Self::UserType {
                $($wrap)?(self)$(.$($op)+)?
            }
        }
        )+
    };
}

impl_toluadata!(
    [u8; u8],
    [u16; u16],
    [u32; u32],
    [i64; i64],
    [bool; bool],
    [String; String],
    [u64; String => to_string()],
    [RoleId; String => 0.to_string()],
    [UserId; String => 0.to_string()],
    [GuildId; String => 0.to_string()],
    [EmojiId; String => 0.to_string()],
    [ChannelId; String => 0.to_string()],
    [MessageId; String => 0.to_string()],
    [StickerId; String => 0.to_string()],
    [WebhookId; String => 0.to_string()],
    [AttachmentId; String => 0.to_string()],
    [DiscordUser; User; User],
    [DiscordRole; Role; Role],
    [DiscordGuild; Guild; Guild],
    [DiscordMember; Member; Member],
    [DiscordMessage; Message; Message],
    [DiscordChannel; Channel; Channel],
    [DiscordTimestamp; Timestamp; Timestamp],
    [DiscordGuildChannel; GuildChannel; GuildChannel],
    [DiscordPrivateChannel; PrivateChannel; PrivateChannel],
    [DiscordChannelCategory; ChannelCategory; ChannelCategory],
);
