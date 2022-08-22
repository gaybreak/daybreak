use time::OffsetDateTime;

use super::Id;

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel
    #thread-member-object-thread-member-structure"
)]
pub struct ThreadMember {
    pub id: Option<Id>,
    pub user_id: Option<Id>,
    pub join_timestamp: OffsetDateTime,
    pub flags: u8,
}
