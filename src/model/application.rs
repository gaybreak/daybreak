use enumflags2::bitflags;

use super::{permission::Permissions, user::User, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/application\
    #application-object-application-structure"
)]
#[derive(Clone, Debug)]
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

#[bitflags]
#[repr(u32)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/application#application-object-application-flags"
)]
#[derive(Clone, Copy, Debug)]
pub enum ApplicationFlags {
    GatewayPresence = 1 << 12,
    GatewayPresenceLimited = 1 << 13,
    GatewayGuildMembers = 1 << 14,
    GatewayGuildMembersLimited = 1 << 15,
    VerificationPendingGuildLimit = 1 << 16,
    Embedded = 1 << 17,
    GatewayMessageContent = 1 << 18,
    GatewayMessageContentLimited = 1 << 19,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/application\
    #install-params-object-install-params-structure"
)]
#[derive(Clone, Debug)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: Permissions,
}

#[doc = discord_url!("https://discord.com/developers/docs/topics/teams#data-models-team-object")]
#[derive(Clone, Debug)]
pub struct Team {
    pub icon: Option<String>,
    pub id: Id,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Id,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/teams#data-models-team-member-object"
)]
#[derive(Clone, Debug)]
pub struct TeamMember {
    pub membership_state: TeamMemberState,
    pub team_id: Id,
    pub user: User,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/teams#data-models-membership-state-enum"
)]
#[derive(Clone, Copy, Debug)]
pub enum TeamMemberState {
    Invited = 1,
    Accepted = 2,
}
