use std::path::Component::ParentDir;
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
        user::User,
        message::*
    },
};
use crate::model::message::Message;

enum MessageQueryParams {   // TODO: MessageID
    Around(Id),
    Before(Id),
    After(Id)
}
// ids?
struct ChannelContext<'ctx>(&'ctx Context);

impl Context {
    /// channel features- passes context into channelcontext
    pub fn channel(&self) -> ChannelContext {
        ChannelContext(self)
    }
}

impl ChannelContext {
    async fn channel_request_id(
        &self,
        channel_id: &Id,
        method: Method,
    ) -> Result<Channel, Error> {
        self.0.empty_request(Request::new(
            Permissions::ViewChannel.into(),
            method,
            format!("/channels/{channel_id}"),
        ))
            .await
    }
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #get-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn get_channel(&self, channel_id: &Id) -> Result<Channel,Error> {
        self.channel_request_id(&channel_id, Method::GET).await
    }

    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #deleteclose-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn channel_delete(&self, channel_id: &Id) -> Result<Channel,Error> {
        self.channel_request_id(&channel_id, Method::DELETE).await
    }

    pub async fn say(&self, channel_id: &Id) -> Result<Message,Error> {
        self.0.
    }
/*
    pub async fn _edit<F: FnOnce(ChannelEdit) -> ChannelEdit>(
        self,
        channel_id: &Id,
        f: F,
    ) -> Result<Channel, Error> {
        let patch = f(ChannelEdit::default());
        self.request_with_params(Request::new(
            Permissions::ViewChannel.into(),
            Method::PATCH,
            format!("/channels/{channel_id}"),
        ),
                                patch
        ).await
    }

 */
    pub async fn edit(
        self,
        channel_id: &Id,
        edit: ChannelEdit,
    ) -> Result<Channel, Error> {
        self.request_with_params(Request::new(
            Permissions::ViewChannel.into(),
            Method::PATCH,
            format!("/channels/{channel_id}"),
        ),
        edit
        ).await
    }
}


/// The little worker people enter the computer and change the channel through here
/// any data passed through here can be built into new or existing channel (build, patch)
/// TODO: finish
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ChannelEdit {
    pub name: Option<String>,
    pub channel_type: Option<ChannelType>,
    pub position: Option<u16>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub rate_limit_per_user: Option<u16>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u8>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<Id>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub default_auto_archive_duration: Option<u16>,
    // pub available_tags: something - presume not included
    // pub default_reaction_emoji
    pub default_thread_rate_limit_per_user: Option<u16>
}

impl ChannelEdit {
    /// Change the name of a channel.
    pub fn name(self, name: &str) -> Self {
        ChannelEdit {
            name: Some(name.to_owned()),
            ..self
        }
    }
    /// TODO: limited to specific conversions.
    pub fn with_type(self, channel_type: ChannelType) -> Self {
        ChannelEdit {
            channel_type: Some(channel_type),
            ..self
        }
    }
    /// Change the position of a channel
    pub fn position(self, position: u16) -> Self {
        ChannelEdit {
            position: Some(position),
            ..self
        }
    }
    /// Channel topic / description
    pub fn topic(self, topic: &str) -> Self {
        ChannelEdit {
            topic: Some(topic.to_owned()),
            ..self
        }
    }
    /// True if NSFW.
    pub fn nsfw(self, nsfw: bool) -> Self {
        ChannelEdit {
            nsfw: Some(nsfw),
            ..self
        }
    }
    /// Sets the rate limit of the channel
    pub fn rate_limit(self, rate_limit: u16) -> Self {
        ChannelEdit {
            rate_limit_per_user: Some(rate_limit),
            ..self
        }
    }
    pub fn bitrate(self, bitrate: u32) -> Self {
        ChannelEdit {
            bitrate: Some(bitrate),
            ..self
        }
    }
    // TODO: check - can only some params be used with voice/thread/forum/text?
    pub fn user_limit(self, lim: u8) -> Self {
        ChannelEdit {
            user_limit: Some(lim),
            ..self
        }
    }
    pub fn overwrite_perms(self, v: Vec<PermissionOverwrite>) -> Self {
        ChannelEdit {
            permission_overwrites: v,
            ..self
        }
    }
    pub fn parent(self, parent: Id) -> Self {
        ChannelEdit {
            parent_id: Some(parent),
            ..self
        }
    }
    pub fn region(self, rtc_region: &str) -> Self {
        ChannelEdit {
            rtc_region: Some(rtc_region.to_owned()),
            ..self
        }
    }
    pub fn video_quality(self, quality_mode: VideoQualityMode) -> Self {
        ChannelEdit {
            video_quality_mode: Some(quality_mode),
            ..self
        }
    }
    pub fn archive_duration(self, duration: u16) -> Self {
        ChannelEdit {
            default_auto_archive_duration: Some(duration),
            ..self
        }
    }
    pub fn thread_rate_limit(self, limit: u16) -> Self {
        ChannelEdit {
            default_thread_rate_limit_per_user: Some(limit),
            ..self
        }
    }
    /// helper to extend for custom behaviour
    pub fn by_fn<F: Fn(&Channel, ChannelEdit) -> ChannelEdit>(self, base_channel: &Channel, f:F) -> Self {
        f(base_channel, self)
    }

    // TODO: we should check on patch if its valid for channel type
    pub fn validate_text(&self) -> bool {
        match self {
            ChannelEdit {
                bitrate: None,
                user_limit: None,
                rtc_region: None,
                video_quality_mode: None,
                ..
            } => true,
            _ => false,
        }
    }
    pub fn validate_announcement(&self) -> bool {
        match self {
            ChannelEdit {
                rate_limit_per_user: None,
                bitrate: None,
                user_limit: None,
                rtc_region: None,
                video_quality_mode: None,
                default_thread_rate_limit_per_user: None,
                ..
            } => true,
            _ => false
        }
    }
    pub fn validate_forum(&self) -> bool {
        match self {
            ChannelEdit {
                channel_type: None,
                bitrate: None,
                user_limit: None,
                rtc_region: None,
                video_quality_mode: None,
                default_auto_archive_duration: None,
                default_thread_rate_limit_per_user: None,
                ..
            } => true,
            _ => false
        }
    }
    pub fn validate_voice(&self) -> bool {
        match self {
            ChannelEdit {
                channel_type: None,
                topic: None,
                rate_limit_per_user: None,
                bitrate: Some(8000..=384000),
                default_auto_archive_duration: None,
                default_thread_rate_limit_per_user: None,
                ..
            } => true,
            _ => false,
        }
    }
    pub fn validate_stage(&self) -> bool {
        match self {
            ChannelEdit {
                channel_type: None,
                topic: None,
                nsfw: None,
                rate_limit_per_user: None,
                bitrate: Some(8000..=64000),
                user_limit: None,
                parent_id: None,
                video_quality_mode: None,
                default_auto_archive_duration: None,
                default_thread_rate_limit_per_user: None,
                ..
            } => true,
            _ => false,
        }
    }


}