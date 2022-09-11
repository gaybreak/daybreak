use anyhow::Error;
use enumflags2::BitFlag;
use hyper::Method;
use serde::{Deserialize, Serialize};

use super::Context;
use crate::model::emoji::GuildEmojis;
use crate::{
    http::Request,
    model::{emoji::Emoji, permission::Permissions, Id},
};

impl Context {
    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#list-guild-emojis")]
    #[doc = http_errors_doc!()]
    pub async fn all_emojis(
        &self,
        guild_id: Id
    ) -> Result<GuildEmojis, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/guilds/{guild_id}/emojis"),
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#get-guild-emoji")]
    #[doc = http_errors_doc!()]
    pub async fn find(
        &self,
        guild_id: Id,
        emoji_id: Id
    ) -> Result<Emoji, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/guilds/{guild_id}/emojis/{emoji_id}"),
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji#create-guild-emoji")]
    #[doc = http_errors_doc!()]
    pub async fn create(
        &self,
        guild_id: Id,
        emoji: ModifyEmojiParams,
    ) -> Result<Emoji, Error> {
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
        emoji: ModifyEmojiParams,
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
    pub async fn delete(
        &self,
        guild_id: Id,
        emoji_id: Id,
    ) -> Result<u16, Error> {
        self.empty_request(Request::new(
            Permissions::ManageEmojisAndStickers.into(),
            Method::DELETE,
            format!("/guilds/{guild_id}/emojis/{emoji_id}"),
        ))
        .await
    }
}

#[doc = discord_url!("https://discord.com/developers/docs/resources/emoji\
    #create-guild-emoji-json-params"
)]
#[doc = discord_url!("https://discord.com/developers/docs/resources/emoji\
    #modify-guild-emoji-json-params"
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ModifyEmojiParams {
    pub name: String,
    pub image: Option<String>,
    pub roles: Vec<Id>,
}

#[doc = discord_url!("https://discord.com/developers/docs/resources/emoji\
    #modify-guild-emoji-json-params"
)]
impl ModifyEmojiParams {
    #[doc = discord_url!("https://discord.com/developers/docs/resources/emoji\
    #create-guild-emoji-json-params"
    )]
    pub fn image(self, image: &str) -> Self {
        Self {
            image: Some(image.to_owned()),
            ..self
        }
    }
}