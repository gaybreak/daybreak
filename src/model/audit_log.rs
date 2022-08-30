use super::{
    auto_moderation::AutoModerationRule, channel::Channel, integration::Integration,
    scheduled_event::ScheduledEvent, user::User, webhook::Webhook, Id,
};

#[doc = discord_url!("https://discord.com/developers/docs/resources/audit-log#audit-logs-resource")]
#[derive(Clone, Debug)]
pub struct AuditLog {
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub auto_moderation_rules: Vec<AutoModerationRule>,
    pub guild_scheduled_events: Vec<ScheduledEvent>,
    pub integrations: Vec<Integration>,
    pub threads: Vec<Channel>,
    pub users: Vec<User>,
    pub webhooks: Vec<Webhook>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/audit-log\
    #audit-log-entry-object-audit-log-entry-structure"
)]
#[derive(Clone, Debug)]
pub struct AuditLogEntry {
    pub target_id: Option<Id>,
    pub changes: Vec<AuditLogChange>,
    pub user_id: Option<Id>,
    pub id: Id,
    pub action_type: AuditLogAction,
    pub options: Option<AuditLogEntryInfo>,
    pub reason: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/audit-log\
    #audit-log-change-object-audit-log-change-structure"
)]
#[derive(Clone, Debug)]
pub struct AuditLogChange {
    pub new_value:?	mixed (matches object field's type)	New value of the key
old_value?	mixed (matches object field's type)	Old value of the key
key	string	
}
