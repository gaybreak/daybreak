use super::{Id, Permissions};

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/permissions#role-object-role-structure"
)]
#[derive(Clone, Debug)]
pub struct Role {
    pub id: Id,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u8,
    pub permissions: Permissions,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure"
)]
#[derive(Clone, Copy, Debug)]
pub struct RoleTags {
    pub bot_id: Option<Id>,
    pub integration_id: Option<Id>,
    pub premium_subscriber: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-role-create-guild-role-create-event-fields"
)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-role-update-guild-role-update-event-fields"
)]
#[derive(Clone, Debug)]
pub struct NewRole {
    pub guild_id: Id,
    pub role: Role,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-role-delete-guild-role-delete-event-fields"
)]
#[derive(Clone, Copy, Debug)]
pub struct DeletedRole {
    pub guild_id: Id,
    pub role_id: Id,
}
