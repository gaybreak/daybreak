use anyhow::Error;
use hyper::Method;
use serde::{Deserialize, Deserializer, Serialize};
use time::OffsetDateTime;

use super::Context;
use crate::{
    http::Request,
    model::{
        channel::*,
        permission::Permissions,
        Id, member::ThreadMember,
        user::User},
};
enum MessageQueryParams {
    Around(Id),
    Before(Id),
    After(Id)
}
impl Channel {
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #get-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn from_id(ctx: &Context, id: &Id) -> Result<Channel,Error> {
        ctx.channel_request_id(id, Method::GET)
    }

    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #deleteclose-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn delete(ctx: &Context, id: &Id) -> Result<Channel,Error> {
        ctx.channel_request_id(id, Method::DELETE)
    }

    pub async fn edit<F: FnOnce(ChannelModifier) -> ChannelModifier>(
        self,
        ctx: &Context,
        f: F,
    ) -> Result<Channel, Error> {
        let id = self.id;
        let patch = f(ChannelModifier::default());
        let reply = ctx.request_with_params(Request::new(
            Permissions::ViewChannel.into(),
            Method::PATCH,
            format!("/channels/{id}"),
        ),
                                patch
        ).await?;
    }
}

impl Context {
    pub async fn channel_request_id(
        &self,
        channel_id: &Id,
        method: Method,
    ) -> Result<Channel, Error> {
        self.empty_request(Request::new(
            Permissions::ViewChannel.into(),
            method,
            format!("/channels/{channel_id}"),
        ))
            .await?
    }


}

/// The little worker people enter the computer and change the channel through here
/// any data passed through here can be built into new or existing channel (build, patch)
/// TODO: finish
#[derive(Default, Debug, Copy, Serialize, Deserialize)]
struct ChannelEdit {
    pub id: Id,
    pub channel_type: ChannelType,
    pub guild_id: Option<Id>,
    pub position: Option<u16>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Id>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u8>,
    pub rate_limit_per_user: Option<u16>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Id>,
    pub application_id: Option<Id>,
    pub parent_id: Option<Id>,
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub message_count: Option<u32>,
    pub member_count: Option<u8>,
    pub thread_metadata: Option<Thread>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u16>,
    pub permissions: Option<Permissions>,
    pub flags: Option<ChannelFlags>,
    pub total_message_sent: Option<u32>,
    #[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#thread-create")]
    pub newly_created: Option<bool>,
}