use super::Id;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-rule-object-auto-moderation-rule-structure"
)]
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

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-rule-object-trigger-types"
)]
pub enum AutoModerationTriggerType {
    Keyword = 1,
    HarmfulLink = 2,
    Spam = 3,
    KeywordPreset = 4,
    MentionSpam = 5,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-rule-object-trigger-metadata"
)]
pub struct AutoModerationTrigger {
    pub keyword_filter: Vec<String>,
    pub presets: Vec<KeywordPresetType>,
    pub allow_list: Vec<String>,
    pub mention_total_limit: u8,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-rule-object-keyword-preset-types"
)]
pub enum KeywordPresetType {
    Profanity = 1,
    SexualContent = 2,
    Slurs = 3,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-rule-object-event-types"
)]
pub enum AutoModerationEvent {
    MessageSend = 1,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-action-object-auto-moderation-action-structure"
)]
pub struct AutoModerationAction {
    pub kind: AutoModerationActionType,
    pub metadata: Option<AutoModerationActionData>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-action-object-action-types"
)]
pub enum AutoModerationActionType {
    BlockMessage = 1,
    SendAlertMessage = 2,
    Timeout = 3,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/auto-moderation
    #auto-moderation-action-object-action-metadata"
)]
pub struct AutoModerationActionData {
    pub channel_id: Id,
    pub duration_seconds: u32,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/gateway
    #auto-moderation-action-execution-auto-moderation-action-execution-event-fields"
)]
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
