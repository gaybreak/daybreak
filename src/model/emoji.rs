use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::model::{user::User, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/emoji#emoji-object-emoji-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Emoji {
    pub id: Option<Id>,
    pub name: Option<String>,
    pub roles: Option<Vec<Id>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sticker {
    pub id: Id,
    pub pack_id: Option<Id>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: Option<String>,
    pub kind: StickerType,
    pub format_type: StickerFormat,
    pub available: Option<bool>,
    pub guild_id: Option<Id>,
    pub user: Option<User>,
    pub sort_value: Option<u8>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-types"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum StickerType {
    Standard = 1,
    Guild = 2,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-format-types"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum StickerFormat {
    Png = 1,
    Apng = 2,
    Lottie = 3,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/sticker\
    #sticker-item-object-sticker-item-structure"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StickerItem {
    pub id: Id,
    pub name: String,
    pub format_type: StickerFormat,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-emojis-update-guild-emojis-update-event-fields"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildEmojis {
    pub guild_id: Id,
    pub emojis: Vec<Emoji>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-stickers-update-guild-stickers-update-event-fields"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildStickers {
    pub guild_id: Id,
    pub stickers: Vec<Sticker>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/emoji#create-guild-emoji-json-params"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEmoji {
    pub name: String,
    pub image: String,
    pub roles: Vec<Id>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/emoji#modify-guild-emoji-json-params"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditEmoji {
    pub name: String,
    pub roles: Option<Vec<Id>>,
}
