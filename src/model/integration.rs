use time::OffsetDateTime;

use super::{user::User, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#integration-object-integration-structure"
)]
#[derive(Clone, Debug)]
pub struct Integration {
    pub id: Id,
    pub name: String,
    pub kind: String,
    pub enabled: Option<bool>,
    pub syncing: Option<bool>,
    pub role_id: Option<Id>,
    pub enable_emoticons: Option<bool>,
    pub expire_behavior: Option<IntegrationExpire>,
    pub expire_grace_period: Option<u16>,
    pub user: Option<User>,
    pub account: IntegrationAccount,
    pub synced_at: Option<OffsetDateTime>,
    pub subscriber_count: Option<u32>,
    pub revoked: Option<bool>,
    pub application: Option<IntegrationApplication>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #integration-create-integration-create-event-additional-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #integration-update-integration-update-event-additional-fields"
    )]
    pub guild_id: Option<Id>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild\
    #integration-object-integration-expire-behaviors"
)]
#[derive(Clone, Copy, Debug)]
pub enum IntegrationExpire {
    RemoveRole = 0,
    Kick = 1,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild\
    #integration-account-object-integration-account-structure"
)]
#[derive(Clone, Debug)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild\
    #integration-application-object-integration-application-structure"
)]
#[derive(Clone, Debug)]
pub struct IntegrationApplication {
    pub id: Id,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub bot: Option<User>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-integrations-update-guild-integrations-update-event-fields"
)]
#[derive(Clone, Copy, Debug)]
pub struct Integrations {
    pub guild_id: Id,
}
