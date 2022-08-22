use super::{application::Application, guild::UnavailableGuild, user::User};

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#hello-hello-structure")]
pub struct Hello {
    pub heartbeat_interval: u32,
}

#[derive(Clone, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/topics/gateway#ready-ready-event-fields")]
pub struct Ready {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u16, u16)>,
    pub application: Application,
}
