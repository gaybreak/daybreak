use time::OffsetDateTime;

use super::Id;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
    #thread-member-object-thread-member-structure"
)]
pub struct ThreadMember<T> {
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
    pub member: Option<T>,
    pub presences: Option<Vec<T>>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #thread-members-update-thread-members-update-event-fields"
)]
pub struct ThreadMembers<T> {
    pub id: Id,
    pub guild_id: Id,
    pub member_count: u8,
    pub added_members: Option<Vec<ThreadMember<T>>>,
    pub removed_member_ids: Option<Vec<Id>>,
}
