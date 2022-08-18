use super::{user::User, Id};

#[doc = discord_url!("https://discord.com/developers/docs/topics/teams#data-models-team-object")]
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
pub struct TeamMember {
    pub membership_state: MemberState,
    pub team_id: Id,
    pub user: User,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/teams#data-models-membership-state-enum"
)]
pub enum MemberState {
    Invited,
    Accepted,
}
