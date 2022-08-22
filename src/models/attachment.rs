use super::Id;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#attachment-object-attachment-structure"
)]
pub struct Attachment {
    pub id: Id,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u32,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub ephemeral: Option<bool>,
}
