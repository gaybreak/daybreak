use crate::models::{user::User, Id};

#[doc = discord_url!(
    "https://discord.com/developers/docs/resources/emoji#emoji-object-emoji-structure"
)]
#[derive(Clone, Debug)]
pub struct Emoji {
    pub id: Id,
    pub name: Option<String>,
    pub roles: Option<Id>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
