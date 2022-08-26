use time::OffsetDateTime;

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-structure"
)]
#[derive(Clone, Debug)]
pub struct Embed {
    pub title: Option<String>,
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<OffsetDateTime>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure"
)]
#[derive(Clone, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
