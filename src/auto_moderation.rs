use anyhow::Error;
use hyper::Method;

use super::Context;
use crate::{
    http::Request,
    model::{auto_moderation::AutoModerationRule, permission::Permissions, Id},
};

impl Context {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/resources/auto-moderation\
        #list-auto-moderation-rules-for-guild"
    )]
    #[doc = http_errors_doc!()]
    pub async fn auto_moderation_rules(
        &self,
        guild_id: Id,
    ) -> Result<Vec<AutoModerationRule>, Error> {
        self.empty_request(Request::new(
            Permissions::ManageGuild.into(),
            Method::GET,
            format!("/guilds/{guild_id}/auto-moderation/rules"),
        ))
        .await
    }

    #[doc = discord_url!(
        "https://discord.com/developers/docs/resources/auto-moderation#get-auto-moderation-rule"
    )]
    #[doc = http_errors_doc!()]
    pub async fn auto_moderation_rule(
        &self,
        guild_id: Id,
        auto_moderation_rule_id: Id,
    ) -> Result<AutoModerationRule, Error> {
        self.empty_request(Request::new(
            Permissions::ManageGuild.into(),
            Method::GET,
            format!("/guilds/{guild_id}/auto-moderation/rules/{auto_moderation_rule_id}"),
        ))
        .await
    }
}
