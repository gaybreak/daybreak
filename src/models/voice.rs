use time::OffsetDateTime;

use super::Id;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/voice#voice-state-object-voice-state-structure"
)]
pub struct VoiceState<T> {
    pub guild_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub user_id: Id,
    pub member: Option<T>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<OffsetDateTime>,
}
