use time::OffsetDateTime;

use super::{
    permission::Permissions,
    presence::{Activity, Presence},
    user::User,
    Id,
};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-structure"
)]
#[derive(Clone, Debug)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Id>,
    pub joined_at: Option<OffsetDateTime>,
    pub premium_since: Option<OffsetDateTime>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub permissions: Option<Permissions>,
    pub communication_disabled_until: Option<OffsetDateTime>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #guild-member-add-guild-member-add-extra-fields"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #guild-member-update-guild-member-update-event-fields"
    )]
    pub guild_id: Option<Id>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #thread-member-object-thread-member-structure"
)]
#[derive(Clone, Debug)]
pub struct ThreadMember {
    pub id: Option<Id>,
    pub user_id: Option<Id>,
    pub join_timestamp: OffsetDateTime,
    pub flags: u8,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway#thread-member-update"
    )]
    pub guild_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #thread-members-update-thread-members-update-event-fields"
    )]
    pub member: Option<Member>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #thread-members-update-thread-members-update-event-fields"
    )]
    pub presences: Option<Vec<Activity>>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #thread-members-update-thread-members-update-event-fields"
)]
#[derive(Clone, Debug)]
pub struct ThreadMembers {
    pub id: Id,
    pub guild_id: Id,
    pub member_count: u8,
    pub added_members: Option<Vec<ThreadMember>>,
    pub removed_member_ids: Option<Vec<Id>>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-member-remove-guild-member-remove-event-fields"
)]
#[derive(Clone, Debug)]
pub struct RemovedMember {
    pub guild_id: Id,
    pub user: User,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #guild-members-chunk-guild-members-chunk-event-fields"
)]
#[derive(Clone, Debug)]
pub struct GuildMembers {
    pub guild_id: Id,
    pub members: Vec<Member>,
    pub chunk_index: u32,
    pub chunk_count: u16,
    pub not_found: Vec<Id>,
    pub presences: Vec<Presence>,
    pub nonce: Option<String>,
}
