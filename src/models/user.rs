use enumflags2::bitflags;

use super::Id;

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/user#user-object-user-structure"
)]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Id,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
}

#[bitflags]
#[repr(u32)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/user#user-object-user-flags"
)]
#[derive(Clone, Copy, Debug)]
pub enum UserFlags {
    Staff = 1 << 0,
    Partner = 1 << 1,
    Hypesquad = 1 << 2,
    BugHunterLevel1 = 1 << 3,
    HypesquadOnlineHouse1 = 1 << 6,
    HypesquadOnlineHouse2 = 1 << 7,
    HypesquadOnlineHouse3 = 1 << 8,
    PremiumEarlySupporter = 1 << 9,
    TeamPseudoUser = 1 << 10,
    BugHunterLevel2 = 1 << 14,
    VerifiedBot = 1 << 16,
    VerifiedDeveloper = 1 << 17,
    CertifiedModerator = 1 << 18,
    BotHttpInteractions = 1 << 19,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/user#user-object-premium-types"
)]
#[derive(Clone, Copy, Debug)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}
