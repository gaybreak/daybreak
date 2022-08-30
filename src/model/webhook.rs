use super::{channel::Channel, guild::Guild, user::User, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-structure"
)]
#[derive(Clone, Debug)]
pub struct Webhook {
    pub id: Id,
    pub kind: WebhookType,
    pub guild_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub user: Option<User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub token: Option<String>,
    pub application_id: Option<Id>,
    pub source_guild: Option<Guild>,
    pub source_channel: Option<Channel>,
    pub url: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-types"
)]
#[derive(Clone, Copy, Debug)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
    Application = 3,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #webhooks-update-webhooks-update-event-fields"
)]
#[derive(Clone, Copy, Debug)]
pub struct UpdatedWebhook {
    pub guild_id: Id,
    pub channel_id: Id,
}
