use crate::models::channel::Channel;
use crate::models::member::Member;
use crate::models::message::{Attachment, Message};
use crate::models::user::User;
use crate::models::Id;
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-interaction-structure"
)]
// the inception of T
pub struct Interaction<T> {
    pub id: Id,
    pub application_id: Id,
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData<T>>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/interactions/receiving-and-responding\
#interaction-object-interaction-type"
)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/interactions/receiving-and-responding\
#interaction-object-interaction-data"
)]
pub enum InteractionData<T> {
    ApplicationCommand(ApplicationCommandData<T>),
    MessageComponent(MessageComponentData),
    ModalSubmit(ModalSubmitData<T>),
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/interactions/receiving-and-responding\
#interaction-object-message-component-data-structure"
)]
pub struct MessageComponentData {
    pub custom_id: String,
    pub component_type: u8,
    pub values: Vec<String>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/interactions/receiving-and-responding\
#interaction-object-application-command-data-data-structure"
)]
pub struct ApplicationCommandData<T> {
    pub id: Id,
    pub name: String,
    pub command_type: u16,
    pub resolved: Option<ResolvedData<T>>,
    // i think this needs application commands done?
    pub options: Option<T>,
    pub guild_id: Option<Id>,
    pub target_id: Option<Id>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
"https://discord.com/developers/docs/interactions/receiving-and-responding\
#interaction-object-resolved-data-structure"
)]
pub struct ResolvedData<T> {
    pub users: Option<HashMap<Id, User>>,
    pub members: Option<HashMap<Id, Member>>,
    pub roles: Option<HashMap<Id, T>>,
    pub channels: Option<HashMap<Id, Channel<T>>>,
    pub messages: Option<HashMap<Id, Message<T>>>,
    pub attachments: Option<HashMap<Id, Attachment>>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-modal-submit-data-structure"
)]
pub struct ModalSubmitData<T> {
    pub custom_id: Id,
    pub components: T,
}
