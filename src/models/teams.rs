use super::{user::User, Id};

#[derive(Clone, Debug)]
#[doc = discord_url!("https://discord.com/developers/docs/topics/teams#data-models-team-object")]
pub struct Team {
    pub icon: Option<String>,
    pub id: Id,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Id,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/teams#data-models-team-member-object"
)]
pub struct TeamMember {
    pub membership_state: MemberState,
    pub team_id: Id,
    pub user: User,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/topics/teams#data-models-membership-state-enum"
)]
pub enum MemberState {
    Invited = 1,
    Accepted = 2,
}
