use time::OffsetDateTime;

use super::{presence::Activity, user::User, Id, Permissions};

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-structure"
)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Id>,
    pub joined_at: OffsetDateTime,
    pub premium_since: Option<OffsetDateTime>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<Permissions>,
    pub communication_disabled_until: Option<OffsetDateTime>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #thread-member-object-thread-member-structure"
)]
pub struct ThreadMember {
    pub id: Option<Id>,
    pub user_id: Option<Id>,
    pub join_timestamp: OffsetDateTime,
    pub flags: u8,
    #[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#thread-member-update")]
    pub guild_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/topics/gateway\
        #thread-members-update-thread-members-update-event-fields"
    )]
    pub member: Option<Member>,
    pub presences: Option<Vec<Activity>>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #thread-members-update-thread-members-update-event-fields"
)]
pub struct ThreadMembers {
    pub id: Id,
    pub guild_id: Id,
    pub member_count: u8,
    pub added_members: Option<Vec<ThreadMember>>,
    pub removed_member_ids: Option<Vec<Id>>,
}
