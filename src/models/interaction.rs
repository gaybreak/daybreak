use std::collections::HashMap;

use super::{
    channel::Channel,
    member::Member,
    message::{Attachment, Message},
    user::User,
    Id, Permissions,
};

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-interaction-structure"
)]
pub struct Interaction<T> {
    pub id: Id,
    pub application_id: Id,
    pub kind: InteractionType,
    pub data: Option<InteractionData<T>>,
    pub guild_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub member: Option<Id>,
    pub user: Option<User>,
    pub token: String,
    pub version: u8,
    pub message: Option<Message<T>>,
    pub app_permissions: Option<Permissions>,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
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
    ApplicationCommand(CommandData<T>),
    MessageComponent(ComponentData<T>),
    ModalSubmit(ModalData<T>),
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-application-command-data-structure"
)]
pub struct CommandData<T> {
    pub id: Id,
    pub name: String,
    pub kind: CommandType,
    pub resolved: Option<ResolvedData<T>>,
    pub options: Option<CommandOption>,
    pub guild_id: Option<Id>,
    pub target_id: Option<Id>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-object-application-command-interaction-data-option-structure"
)]
pub enum CommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-object-application-command-interaction-data-option-structure"
)]
pub struct CommandOption {
    pub name: String,
    pub kind: CommandOptionType,
    pub value: Option<String>,
    pub options: Option<Vec<CommandOption>>,
    pub focused: Option<bool>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-object-application-command-option-type"
)]
pub enum CommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
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
    #interaction-object-message-component-data-structure"
)]
pub struct ComponentData<T> {
    pub custom_id: String,
    pub component_type: ComponentType,
    pub values: Vec<T>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/message-components\
    #component-object-component-types"
)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
    TextInput = 4,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-modal-submit-data-structure"
)]
pub struct ModalData<T> {
    pub custom_id: Id,
    pub components: Vec<T>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #message-interaction-object-message-interaction-structure"
)]
pub struct MessageInteraction {
    pub id: Id,
    pub kind: InteractionType,
    pub name: String,
    pub user: User,
    pub member: Option<Member>,
}

#[derive(Clone, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-permissions-object-guild-application-command-permissions-structure"
)]
pub struct CommandPermissions {
    pub id: Id,
    pub application_id: Id,
    pub guild_id: Id,
    pub permissions: Vec<CommandPermission>,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-permissions-object-application-command-permissions-structure"
)]
pub struct CommandPermission {
    pub id: Id,
    pub kind: CommandPermissionType,
    pub permission: bool,
}

#[derive(Clone, Copy, Debug)]
#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-permissions-object-application-command-permissions-structure"
)]
pub enum CommandPermissionType {
    Role = 1,
    User = 2,
    Channel = 3,
}
