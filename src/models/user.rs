use bitflags::bitflags;

use super::Id;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/user#user-object-user-structure"
)]
pub struct User {
    pub id: Id,
    pub username: String,
    pub discriminator: String,
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

bitflags! {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/resources/user#user-object-user-flags"
    )]
    pub struct UserFlags: u32 {
        const STAFF = 1 << 0;
        const PARTNER = 1 << 1;
        const HYPESQUAD = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        const TEAM_PSEUDO_USER = 1 << 10;
        const BUG_HUNTER_LEVEL_2 =  1 <<14;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_DEVELOPER = 1 << 17;
        const CERTIFIED_MODERATOR = 1 << 18;
        const BOT_HTTP_INTERACTIONS =  1 <<19;
    }
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/user#user-object-premium-types"
)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}
