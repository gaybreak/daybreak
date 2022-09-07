use anyhow::Error;
use hyper::Method;

use super::Context;
use crate::{
    http::Request,
    model::{channel::Channel, permission::Permissions, Id},
};

impl Context {
    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #get-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn channel_by_id(
        &self,
        channel_id: Id,
    ) -> Result<Channel, Error> {
        self.empty_request(Request::new(
            Permissions::ViewChannel.into(),
            Method::GET,
            format!("/channels/{channel_id}"),
        ))
            .await
    }

    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #modify-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn channel_modify(
        &self,
        channel_id: Id,
    ) -> Result<Channel, Error> {
        //             Permissions::ViewChannel.into(), Method::PATCH
        unimplemented!()    // TODO: implement json params thingy (ew)
    }

    #[doc = discord_url!(
    "https://discord.com/developers/docs/resources/channel\
        #deleteclose-channel"
    )]
    #[doc = http_errors_doc!()]
    pub async fn channel_delete(
        &self,
        channel_id: Id,
    ) -> Result<Channel, Error> {
        self.empty_request(Request::new(
            Permissions::ViewChannel.into(),
            Method::DELETE,
            format!("/channels/{channel_id}"),
        ))
            .await
    }
}

