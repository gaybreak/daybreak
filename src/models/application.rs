use bitflags::bitflags;

use super::{teams::Team, user::User, Id};

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/application\
    #application-object-application-structure"
)]
pub struct Application {
    pub id: Id,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<Id>,
    pub primary_sku_id: Option<Id>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<ApplicationFlags>,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<InstallParams>,
    pub custom_install_url: Option<String>,
}

bitflags! {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/resources/application\
        #application-object-application-flags"
    )]
    pub struct ApplicationFlags: u32 {
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED  = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED= 1 << 19;
    }
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/application\
    #install-params-object-install-params-structure"
)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}
