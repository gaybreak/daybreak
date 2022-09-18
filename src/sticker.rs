use std::fs::File;
use anyhow::Error;
use enumflags2::BitFlag;
use hyper::Method;
use serde::{Serialize, Deserialize};

use crate::http::Request;
use crate::model::{Id, emoji::{Sticker, StickerPack}, permission::Permissions};
use super::Context;

impl Context {
    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker#get-sticker")]
    #[doc = http_errors_doc!()]
    pub async fn get_sticker(
        &self,
        sticker_id: Id
    ) -> Result<Sticker, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/stickers/{sticker_id}")
        ))
        .await
    }

    /// # Returns
    /// A Vec<StickerPack> containing all sticker packs,
    /// or an empty Vec<T> if there is an HTTP error.
    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #list-nitro-sticker-packs"
    )]
    #[doc = http_errors_doc!()]
    pub async fn list_nitro_sticker_packs(
        &self
    ) -> Vec<StickerPack> {
        self.nitro_sticker_packs().await.unwrap_or_default().sticker_packs
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #list-nitro-sticker-packs"
    )]
    #[doc = http_errors_doc!()]
    async fn nitro_sticker_packs(
        &self
    ) -> Result<StickerPacksResponse, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("sticker-packs")
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #list-guild-stickers"
    )]
    #[doc = http_errors_doc!()]
    pub async fn list_guild_stickers(
        &self,
        guild_id: Id
    ) -> Result<Vec<Sticker>, Error> {
        self.empty_request(Request::new(
            // A bit ambiguous, because you require Permissions::ManageEmojisAndStickers to receive
            // the "user" field, but I guess Discord handles that.
            Permissions::empty(),
            Method::GET,
            format!("/guilds/{guild_id}/stickers")
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #get-guild-sticker"
    )]
    #[doc = http_errors_doc!()]
    pub async fn get_guild_sticker(
        &self,
        guild_id: Id,
        sticker_id: Id,
    ) -> Result<Sticker, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/guilds/{guild_id}/stickers")
        ))
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #create-guild-sticker"
    )]
    #[doc = http_errors_doc!()]
    pub async fn create_sticker(
        &self,
        guild_id: Id,
        sticker: CreateSticker,
    ) -> Result<Sticker, Error> {
        self.request_with_params(Request::new(
            Permissions::ManageEmojisAndStickers.into(),
            Method::POST,
            format!("/guilds/{guild_id}/stickers")
        ), sticker)
        .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #modify-guild-sticker"
    )]
    #[doc = http_errors_doc!()]
    pub async fn modify_sticker(
        &self,
        guild_id: Id,
        sticker: EditSticker,
    ) -> Result<Sticker, Error> {
        self.request_with_params(Request::new(
            Permissions::ManageEmojisAndStickers.into(),
            Method::PATCH,
            format!("/guilds/{guild_id}/stickers")
        ), sticker)
            .await
    }

    #[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #delete-guild-sticker"
    )]
    #[doc = http_errors_doc!()]
    pub async fn delete_sticker(
        &self,
        guild_id: Id,
        sticker_id: Id,
    ) -> Result<u16, Error> {
        self.empty_request(Request::new(
            Permissions::ManageEmojisAndStickers.into(),
            Method::DELETE,
            format!("/guilds/{guild_id}/stickers/{sticker_id}")
        ))
        .await
    }
}

#[doc = discord_url!("https://discord.com/developers/docs/resources/sticker\
    #list-nitro-sticker-packs-response-structure"
)]
#[derive(Serialize, Deserialize, Debug)]
struct StickerPacksResponse {
    pub sticker_packs: Vec<StickerPack>
}

impl Default for StickerPacksResponse {
    fn default() -> Self {
        Self {
            sticker_packs: Vec::new()
        }
    }
}

struct CreateSticker {
    name: String,
    description: String,
    tags: String,
    file: File
}

struct EditSticker {
    name: Option<String>,
    description: Option<String>,
    tags: Option<String>,
}

impl EditSticker {
    pub fn name(self, name: &str) -> Self {
        EditSticker {
            name: Some(name.to_owned()),
            ..self
        }
    }

    pub fn desc(self, desc: &str) -> Self {
        EditSticker {
            description: Some(desc.to_owned()),
            ..self
        }
    }

    pub fn tags(self, tags: &str) -> Self {
        EditSticker {
            tags: Some(tags.to_owned()),
            ..self
        }
    }
}