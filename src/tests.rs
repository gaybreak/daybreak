#![allow(clippy::unwrap_used)]

use enumflags2::BitFlag;
use once_cell::sync::Lazy;
use time::{Date, Month, PrimitiveDateTime, Time};

use crate::{
    model::{
        channel::{
            Channel, ChannelFlags, ChannelType, PermissionOverwrite, PermissionOverwriteKind,
        },
        emoji::{Emoji, Sticker, StickerFormat, StickerType},
        guild::{
            ExplicitContentFilterLevel, Guild, MessageNotificationLevel, MfaLevel, NsfwLevel,
            PremiumTier, SystemChannelFlags, VerificationLevel, WelcomeScreen,
            WelcomeScreenChannel,
        },
        permission::Permissions,
        role::{Role, RoleTags},
        user::{User, UserFlags},
        Id,
    },
    Context, ContextConfig,
};

static CTX: Lazy<Context> = Lazy::new(|| {
    Context::new(&ContextConfig {
        token: env!("DAYBREAK_BOT_TOKEN"),
    })
});

#[allow(clippy::too_many_lines)]
fn guild() -> Guild {
    let user_lara = User {
        id: Id(258_568_289_746_288_641),
        username: Some("laralove".to_owned()),
        discriminator: Some("0416".to_owned()),
        avatar: Some("a_7025c0b820083516aa701e8e28a8de3a".to_owned()),
        bot: None,
        system: None,
        mfa_enabled: None,
        banner: None,
        accent_color: None,
        locale: None,
        verified: None,
        email: None,
        flags: None,
        premium_type: None,
        public_flags: Some(UserFlags::HypesquadOnlineHouse3),
        member: None,
    };

    Guild {
        id: Id(1_015_676_961_814_749_265),
        name: "Daybreak".to_owned(),
        icon: Some("757c2962467bdbf77cf1db067226803e".to_owned()),
        icon_hash: None,
        splash: None,
        discovery_splash: None,
        owner: Some(false),
        owner_id: Id(258_568_289_746_288_641),
        permissions: None,
        region: None,
        afk_channel_id: None,
        afk_timeout: 300,
        widget_enabled: Some(true),
        widget_channel_id: Some(Id(1_015_677_691_946_606_652)),
        verification_level: VerificationLevel::Medium,
        default_message_notifications: MessageNotificationLevel::OnlyMentions,
        explicit_content_filter: ExplicitContentFilterLevel::AllMembers,
        roles: vec![
            Role {
                id: Id(1_015_682_177_050_083_352),
                name: "Daybreak".to_owned(),
                color: 0x007c_b8f9,
                hoist: false,
                icon: None,
                unicode_emoji: None,
                position: 0,
                permissions: crate::model::permission::Permissions::Administrator,
                managed: true,
                mentionable: false,
                tags: Some(RoleTags {
                    bot_id: Some(Id(1_015_681_541_101_342_730)),
                    integration_id: Some(Id(1_015_681_541_101_342_730)),
                    premium_subscriber: Some(false),
                }),
            },
            Role {
                id: Id(1_015_677_267_063_619_654),
                name: "team".to_owned(),
                color: 0x00f7_c944,
                hoist: true,
                icon: None,
                unicode_emoji: None,
                position: 1,
                permissions: Permissions::Administrator,
                managed: false,
                mentionable: true,
                tags: None,
            },
        ],
        emojis: vec![
            Emoji {
                id: Some(Id(1_019_343_476_447_518_730)),
                name: Some("githubcat".to_owned()),
                roles: Some(vec![]),
                user: Some(user_lara.clone()),
                require_colons: Some(true),
                managed: Some(false),
                animated: Some(true),
                available: Some(true),
            },
            Emoji {
                id: Some(Id(1_019_343_598_518_538_280)),
                name: Some("gaybreak".to_owned()),
                roles: Some(vec![]),
                user: Some(user_lara.clone()),
                require_colons: Some(true),
                managed: Some(false),
                animated: Some(false),
                available: Some(true),
            },
        ],
        features: vec![
            "AUTO_MODERATION".to_owned(),
            "COMMUNITY".to_owned(),
            "NEWS".to_owned(),
            "WELCOME_SCREEN_ENABLED".to_owned(),
        ],
        mfa_level: MfaLevel::None,
        application_id: None,
        system_channel_id: Some(Id(1_015_680_953_001_189_488)),
        system_channel_flags: SystemChannelFlags::empty(),
        rules_channel_id: Some(Id(1_015_684_146_506_506_270)),
        max_presences: None,
        max_members: None,
        vanity_url_code: None,
        description: None,
        banner: None,
        premium_tier: PremiumTier::None,
        premium_subscription_count: Some(0),
        preferred_locale: "en-US".to_owned(),
        public_updates_channel_id: Some(Id(1_015_680_953_001_189_488)),
        max_video_channel_users: None,
        approximate_member_count: None,
        approximate_presence_count: None,
        welcome_screen: Some(WelcomeScreen {
            description: Some(
                "The server for anything about Daybreak, a Discord framework in Rust".to_owned(),
            ),
            welcome_channels: vec![WelcomeScreenChannel {
                channel_id: Id(1_015_677_691_946_606_652),
                description: "Get help with using Daybreak".to_owned(),
                emoji_id: None,
                emoji_name: Some("sos".to_owned()),
            }],
        }),
        nsfw_level: NsfwLevel::Safe,
        stickers: Some(vec![Sticker {
            id: Id(1_019_700_662_470_201_386),
            pack_id: None,
            name: "daybreak".to_owned(),
            description: Some("The Daybreak logo in all its glory!".to_owned()),
            tags: "".to_owned(),
            asset: None,
            kind: StickerType::Guild,
            format_type: StickerFormat::Png,
            available: Some(true),
            guild_id: Some(Id(1_015_676_961_814_749_265)),
            user: Some(user_lara),
            sort_value: Some(1),
        }]),
        premium_progress_bar_enabled: false,
        joined_at: Some(
            PrimitiveDateTime::new(
                Date::from_calendar_date(2022, Month::September, 3).unwrap(),
                Time::from_hms(17, 58, 38).unwrap(),
            )
            .assume_utc(),
        ),
        large: Some(false),
        unavailable: Some(false),
        member_count: Some(4),
        voice_states: Some(vec![]),
        members: None,
        channels: Some(vec![Channel {
            id: Id(1_015_684_146_506_506_270),
            channel_type: ChannelType::GuildText,
            guild_id: Some(Id(1_015_676_961_814_749_265)),
            position: Some(0),
            permission_overwrites: Some(vec![PermissionOverwrite {
                id: Id(1_015_684_146_506_506_270),
                kind: PermissionOverwriteKind::Role,
                allow: Permissions::empty(),
                deny: Permissions::SendMessages.into(),
            }]),
            name: Some("rules".to_owned()),
            topic: None,
            nsfw: Some(false),
            last_message_id: Some(Id(1_015_686_214_940_110_848)),
            bitrate: None,
            user_limit: None,
            rate_limit_per_user: None,
            recipients: None,
            icon: None,
            owner_id: None,
            application_id: None,
            parent_id: None,
            last_pin_timestamp: None,
            rtc_region: None,
            video_quality_mode: None,
            message_count: None,
            member_count: None,
            thread_metadata: None,
            member: None,
            default_auto_archive_duration: None,
            permissions: None,
            flags: Some(ChannelFlags::empty()),
            total_message_sent: None,
            newly_created: None,
        }]),
        threads: None,
        presences: None,
        stage_instances: None,
        guild_scheduled_events: None,
    }
}

/// Tests for HTTP endpoints
mod http;
/// Tests for models
mod model;

#[test]
fn context_new() {
    assert_eq!(
        Context::new(&ContextConfig { token: "foo" }).token,
        "Bot foo"
    );
}
