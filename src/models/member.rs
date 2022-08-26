use time::OffsetDateTime;

use super::{presence::Activity, user::User, Id, Permissions};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-structure"
)]
#[derive(Clone, Debug)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Id>,
    pub joined_at: OffsetDateTime,
    pub premium_since: Option<OffsetDateTime>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub permissions: Option<Permissions>,
    pub communication_disabled_until: Option<OffsetDateTime>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #thread-member-object-thread-member-structure"
)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#thread-member-update"
)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #thread-members-update-thread-members-update-event-fields"
)]
#[derive(Clone, Debug)]
pub struct ThreadMember {
    pub id: Option<Id>,
    pub user_id: Option<Id>,
    pub join_timestamp: OffsetDateTime,
    pub flags: u8,
    pub guild_id: Option<Id>,
    pub member: Option<Member>,
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
