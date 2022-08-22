use super::Id;

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/guild#unavailable-guild-object"
)]
pub struct UnavailableGuild {
    pub id: Id,
    pub unavailable: bool,
}
