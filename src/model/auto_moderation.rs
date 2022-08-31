use serde::Deserialize;

use super::Id;

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-rule-object-auto-moderation-rule-structure"
)]
#[derive(Clone, Debug, Deserialize)]
pub struct AutoModerationRule {
    pub id: Id,
    pub guild_id: Id,
    pub name: String,
    pub creator_id: Id,
    pub event_type: AutoModerationEvent,
    pub trigger_type: AutoModerationTriggerType,
    pub trigger_metadata: AutoModerationTrigger,
    pub actions: Vec<AutoModerationAction>,
    pub enabled: bool,
    pub exempt_roles: Vec<Id>,
    pub exempt_channels: Vec<Id>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-rule-object-event-types"
)]
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AutoModerationEvent {
    MessageSend = 1,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-rule-object-trigger-types"
)]
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AutoModerationTriggerType {
    Keyword = 1,
    HarmfulLink = 2,
    Spam = 3,
    KeywordPreset = 4,
    MentionSpam = 5,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-rule-object-trigger-metadata"
)]
#[derive(Clone, Debug, Deserialize)]
pub struct AutoModerationTrigger {
    pub keyword_filter: Vec<String>,
    pub presets: Vec<KeywordPresetType>,
    pub allow_list: Vec<String>,
    pub mention_total_limit: u8,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-rule-object-keyword-preset-types"
)]
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum KeywordPresetType {
    Profanity = 1,
    SexualContent = 2,
    Slurs = 3,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-action-object-auto-moderation-action-structure"
)]
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct AutoModerationAction {
    pub kind: AutoModerationActionType,
    pub metadata: Option<AutoModerationActionData>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-action-object-action-types"
)]
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AutoModerationActionType {
    BlockMessage = 1,
    SendAlertMessage = 2,
    Timeout = 3,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation\
    #auto-moderation-action-object-action-metadata"
)]
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct AutoModerationActionData {
    pub channel_id: Id,
    pub duration_seconds: u32,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway\
    #auto-moderation-action-execution-auto-moderation-action-execution-event-fields"
)]
#[derive(Clone, Debug)]
pub struct AutoModerationExecution {
    pub guild_id: Id,
    pub action: AutoModerationAction,
    pub rule_id: Id,
    pub rule_trigger_type: AutoModerationTriggerType,
    pub user_id: Id,
    pub channel_id: Option<Id>,
    pub message_id: Option<Id>,
    pub alert_system_message_id: Option<Id>,
    pub content: String,
    pub matched_keyword: Option<String>,
    pub matched_content: Option<String>,
}
