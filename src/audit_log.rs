use super::Context;
use crate::model::Id;

impl Context {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/resources/audit-log#get-guild-audit-log"
    )]
    async fn audit_log(guild_id: Id, params: AuditLogParams) {}
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/audit-log#get-guild-audit-log-query-string-params"
)]
#[derive(Clone, Copy, Debug)]
pub struct AuditLogParams {
    pub user_id: Option<Id>,
    pub action_type: Option<AuditLogEvent>,
    pub before: Option<Id>,
    pub limit: Option<u8>,
}
