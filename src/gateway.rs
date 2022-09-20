use anyhow::Error;
use enumflags2::BitFlag;
use hyper::Method;

use crate::Context;
use crate::http::Request;
use crate::model::{permission::Permissions, gateway::BotGateway};

impl Context {
    #[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#get-gateway")]
    #[doc = http_errors_doc!()]
    pub async fn get_gateway(self) -> Result<String, Error> {
        self.empty_request(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("/gateway")
        ))
        .await
    }
    #[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#get-gateway-bot")]
    #[doc = http_errors_doc!()]
    pub async fn get_gateway_bot(self) -> Result<BotGateway, Error> {
        self.request_with_params(Request::new(
            Permissions::empty(),
            Method::GET,
            format!("gateway/bot/")
        ), &self.token)
        .await
    }
}