use anyhow::Error;
use enumflags2::BitFlag;
use hyper::Method;
use serde::{Serialize, Deserialize};

use super::Context;
use crate::{
    http::Request,
    model::{
        emoji::Emoji,
        permission::Permissions,
        Id,
    },
};

impl Context {
    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#list-guild-emojis")]
    #[doc = http_errors_doc!()]
    pub async fn emojis(&self, guild_id: Id) -> Result<Vec<Emoji>, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/guilds/{guild_id}/emojis"),
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#get-guild-emoji")]
    #[doc = http_errors_doc!()]
    pub async fn emoji(&self, guild_id: Id, emoji_id: Id) -> Result<Emoji, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/guilds/{guild_id}/emojis/{emoji_id}"),
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#create-guild-emoji")]
    #[doc = http_errors_doc!()]
    pub async fn create(&self, guild_id: Id, emoji: CreateEmoji) -> Result<Emoji, Error> {
        self.request_with_params(
            Request::new(
                Permissions::ManageEmojisAndStickers.into(),
                Method::POST,
                format!("/guilds/{guild_id}/emojis"),
            ),
            emoji,
        )
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#modify-guild-emoji")]
    #[doc = http_errors_doc!()]
    pub async fn modify(
        &self,
        guild_id: Id,
        emoji_id: Id,
        emoji: EditEmoji,
    ) -> Result<Emoji, Error> {
        self.request_with_params(
            Request::new(
                Permissions::ManageEmojisAndStickers.into(),
                Method::PATCH,
                format!("/guilds/{guild_id}/emojis/{emoji_id}"),
            ),
            emoji,
        )
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#delete-guild-emoji")]
    #[doc = http_errors_doc!()]
    pub async fn delete(&self, guild_id: Id, emoji_id: Id) -> Result<u16, Error> {
        self.empty_request(Request::new(
            Permissions::ManageEmojisAndStickers.into(),
            Method::DELETE,
            format!("/guilds/{guild_id}/emojis/{emoji_id}"),
        ))
        .await
    }
}

#[doc = discord_url ! (
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