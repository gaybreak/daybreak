use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use super::{application::Application, user::User, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#invite-create-invite-create-event-fields"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invite {
    pub channel_id: Id,
    pub code: String,
    pub created_at: OffsetDateTime,
    pub guild_id: Option<Id>,
    pub inviter: Option<User>,
    pub max_age: u32,
    pub max_uses: u16,
    pub target_type: Option<InviteTarget>,
    pub target_user: Option<User>,
    pub target_application: Option<Application>,
    pub temporary: bool,
    pub uses: u8,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types"
)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum InviteTarget {
    Stream = 1,
    EmbeddedApplication = 2,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway#invite-delete-invite-delete-event-fields"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletedInvite {
    pub channel_id: Id,
    pub guild_id: Option<Id>,
    pub code: String,
}
