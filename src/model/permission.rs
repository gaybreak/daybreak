use std::fmt::Display;

use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[repr(u64)]
#[doc =discord_url!(
    "https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags"
)]
#[derive(Clone, Copy, Debug)]
pub enum Permissions {
    CreateInstantInvite = 1 << 0,
    KickMembers = 1 << 1,
    BanMembers = 1 << 2,
    Administrator = 1 << 3,
    ManageChannels = 1 << 4,
    ManageGuild = 1 << 5,
    AddReactions = 1 << 6,
    ViewAuditLog = 1 << 7,
    PrioritySpeaker = 1 << 8,
    Stream = 1 << 9,
    ViewChannel = 1 << 10,
    SendMessages = 1 << 11,
    SendTtsMessages = 1 << 12,
    ManageMessages = 1 << 13,
    EmbedLinks = 1 << 14,
    AttachFiles = 1 << 15,
    ReadMessageHistory = 1 << 16,
    MentionEveryone = 1 << 17,
    UseExternalEmojis = 1 << 18,
    ViewGuildInsights = 1 << 19,
    Connect = 1 << 20,
    Speak = 1 << 21,
    MuteMembers = 1 << 22,
    DeafenMembers = 1 << 23,
    MoveMembers = 1 << 24,
    UseVad = 1 << 25,
    ChangeNickname = 1 << 26,
    ManageNicknames = 1 << 27,
    ManageRoles = 1 << 28,
    ManageWebhooks = 1 << 29,
    ManageEmojisAndStickers = 1 << 30,
    UseApplicationCommands = 1 << 31,
    RequestToSpeak = 1 << 32,
    ManageEvents = 1 << 33,
    ManageThreads = 1 << 34,
    CreatePublicThreads = 1 << 35,
    CreatePrivateThreads = 1 << 36,
    UseExternalStickers = 1 << 37,
    SendMessagesInThreads = 1 << 38,
    UseEmbeddedActivities = 1 << 39,
    ModerateMembers = 1 << 40,
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::CreateInstantInvite => "Create Invite",
            Self::KickMembers => "Kick Members",
            Self::BanMembers => "Ban Members",
            Self::Administrator => "Administrator",
            Self::ManageChannels => "Manage Channels",
            Self::ManageGuild => "Manage Server",
            Self::AddReactions => "Add Reactions",
            Self::ViewAuditLog => "View Audit Log",
            Self::PrioritySpeaker => "Priority Speaker",
            Self::Stream => "Video",
            Self::ViewChannel => "View Channels",
            Self::SendMessages => "Send Messages",
            Self::SendTtsMessages => "Send Text-to-Speech Messages",
            Self::ManageMessages => "Manage Messages",
            Self::EmbedLinks => "Embed Links",
            Self::AttachFiles => "Attach Files",
            Self::ReadMessageHistory => "Read Message History",
            Self::MentionEveryone => "Mention @everyone, @here, and All Roles",
            Self::UseExternalEmojis => "Use External Emoji",
            Self::ViewGuildInsights => "View Server Insights",
            Self::Connect => "Connect",
            Self::Speak => "Speak",
            Self::MuteMembers => "Mute Members",
            Self::DeafenMembers => "Deafen Members",
            Self::MoveMembers => "Move Members",
            Self::UseVad => "Use Voice Activity",
            Self::ChangeNickname => "Change Nickname",
            Self::ManageNicknames => "Manage Nicknames",
            Self::ManageRoles => "Manage Roles",
            Self::ManageWebhooks => "Manage Webhooks",
            Self::ManageEmojisAndStickers => "Manage Emoji and Stickers",
            Self::UseApplicationCommands => "Use Application Commands",
            Self::RequestToSpeak => "Request to Speak",
            Self::ManageEvents => "Manage Events",
            Self::ManageThreads => "Manage Threads",
            Self::CreatePublicThreads => "Create Public Threads",
            Self::CreatePrivateThreads => "Create Private Threads",
            Self::UseExternalStickers => "Use External Stickers",
            Self::SendMessagesInThreads => "Send Messages in Threads",
            Self::UseEmbeddedActivities => "Use Activities",
            Self::ModerateMembers => "Timeout Members",
        };
        f.write_str(s)
    }
}

#[must_use]
pub fn to_pretty_string(permissions: BitFlags<Permissions>) -> String {
    permissions
        .iter()
        .map(|perm| format!("- {perm}"))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use enumflags2::BitFlag;

    use super::Permissions;

    #[test]
    fn to_pretty_string() {
        assert_eq!(
            super::to_pretty_string(Permissions::CreateInstantInvite | Permissions::KickMembers),
            "- Create Invite\n- Kick Members"
        );
    }
}
