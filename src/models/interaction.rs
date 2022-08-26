use std::collections::HashMap;

use super::{
    channel::Channel,
    emoji::Emoji,
    member::Member,
    message::{Attachment, Message},
    user::User,
    Id, Permissions,
};

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-interaction-structure"
)]
#[derive(Clone, Debug)]
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

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-interaction-type"
)]
#[derive(Clone, Copy, Debug)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-interaction-data"
)]
#[derive(Clone, Debug)]
pub struct InteractionData<T> {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub name: Option<String>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub kind: Option<CommandType>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub resolved: Option<ResolvedData<T>>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub options: Option<Vec<CommandOption>>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub guild_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-application-command-data-structure"
    )]
    pub target_id: Option<Id>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-message-component-data-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-modal-submit-data-structure"
    )]
    pub custom_id: Option<String>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-message-component-data-structure"
    )]
    pub component_type: Option<ComponentType>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-message-component-data-structure"
    )]
    pub values: Option<Vec<SelectOption>>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/receiving-and-responding\
        #interaction-object-modal-submit-data-structure"
    )]
    pub components: Option<Vec<Component>>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-object-application-command-interaction-data-option-structure"
)]
#[derive(Clone, Copy, Debug)]
pub enum CommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-object-application-command-interaction-data-option-structure"
)]
#[derive(Clone, Debug)]
pub struct CommandOption {
    pub name: String,
    pub kind: CommandOptionType,
    pub value: Option<String>,
    pub options: Option<Vec<CommandOption>>,
    pub focused: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-object-application-command-option-type"
)]
#[derive(Clone, Copy, Debug)]
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

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #interaction-object-resolved-data-structure"
)]
#[derive(Clone, Debug)]
pub struct ResolvedData<T> {
    pub users: Option<HashMap<Id, User>>,
    pub members: Option<HashMap<Id, Member>>,
    pub roles: Option<HashMap<Id, T>>,
    pub channels: Option<HashMap<Id, Channel<T>>>,
    pub messages: Option<HashMap<Id, Message<T>>>,
    pub attachments: Option<HashMap<Id, Attachment>>,
}

#[doc = fields_documented!()]
#[derive(Clone, Debug)]
pub struct Component {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub kind: ComponentType,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub style: Option<ComponentStyle>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub label: Option<String>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    pub emoji: Option<Emoji>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub custom_id: Option<String>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    pub url: Option<String>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    pub disabled: Option<bool>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub placeholder: Option<String>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    pub options: Vec<SelectOption>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    pub min_values: Option<u8>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #select-menu-object-select-menu-structure"
    )]
    pub max_values: Option<u8>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub min_length: Option<u16>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub max_length: Option<u16>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub required: Option<bool>,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-structure"
    )]
    pub value: Option<String>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/message-components\
    #component-object-component-types"
)]
#[derive(Clone, Copy, Debug)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
    TextInput = 4,
}

#[doc = variants_documented!()]
#[derive(Clone, Copy, Debug)]
pub enum ComponentStyle {
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-styles"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-styles"
    )]
    Primary = 1,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-styles"
    )]
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #text-inputs-text-input-styles"
    )]
    Secondary = 2,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-styles"
    )]
    Success = 3,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-styles"
    )]
    Danger = 4,
    #[doc = discord_url!(
        "https://discord.com/developers/docs/interactions/message-components\
        #button-object-button-styles"
    )]
    Link = 5,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/message-components\
    #select-menu-object-select-option-structure"
)]
#[derive(Clone, Debug)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/receiving-and-responding\
    #message-interaction-object-message-interaction-structure"
)]
#[derive(Clone, Debug)]
pub struct MessageInteraction {
    pub id: Id,
    pub kind: InteractionType,
    pub name: String,
    pub user: User,
    pub member: Option<Member>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-permissions-object-guild-application-command-permissions-structure"
)]
#[derive(Clone, Debug)]
pub struct CommandPermissions {
    pub id: Id,
    pub application_id: Id,
    pub guild_id: Id,
    pub permissions: Vec<CommandPermission>,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-permissions-object-application-command-permissions-structure"
)]
#[derive(Clone, Copy, Debug)]
pub struct CommandPermission {
    pub id: Id,
    pub kind: CommandPermissionType,
    pub permission: bool,
}

#[doc = discord_url!(
    "https://discord.com/developers/docs/interactions/application-commands\
    #application-command-permissions-object-application-command-permissions-structure"
)]
#[derive(Clone, Copy, Debug)]
pub enum CommandPermissionType {
    Role = 1,
    User = 2,
    Channel = 3,
}
