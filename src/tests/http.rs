use super::{guild_id, CTX};
use crate::model::{
    auto_moderation::{
        AutoModerationAction, AutoModerationActionData, AutoModerationActionType,
        AutoModerationEvent, AutoModerationRule, AutoModerationTrigger, AutoModerationTriggerType,
        KeywordPresetType,
    },
    Id,
};

fn mention_spam_rule() -> AutoModerationRule {
    AutoModerationRule {
        id: Id(1_015_689_694_572_466_219),
        guild_id: guild_id(),
        name: "Block Mention Spam".to_owned(),
        creator_id: Id(258_568_289_746_288_641),
        event_type: AutoModerationEvent::MessageSend,
        trigger_type: AutoModerationTriggerType::MentionSpam,
        trigger_metadata: AutoModerationTrigger {
            keyword_filter: None,
            presets: None,
            allow_list: None,
            mention_total_limit: Some(7),
        },
        actions: vec![
            AutoModerationAction {
                kind: AutoModerationActionType::BlockMessage,
                metadata: Some(AutoModerationActionData {
                    channel_id: None,
                    duration_seconds: None,
                }),
            },
            AutoModerationAction {
                kind: AutoModerationActionType::Timeout,
                metadata: Some(AutoModerationActionData {
                    channel_id: None,
                    duration_seconds: Some(60),
                }),
            },
        ],
        enabled: true,
        exempt_roles: vec![],
        exempt_channels: vec![],
    }
}

fn spam_rule() -> AutoModerationRule {
    AutoModerationRule {
        id: Id(1_015_703_423_632_560_241),
        guild_id: guild_id(),
        name: "Block Spam Content".to_owned(),
        creator_id: Id(258_568_289_746_288_641),
        event_type: AutoModerationEvent::MessageSend,
        trigger_type: AutoModerationTriggerType::Spam,
        trigger_metadata: AutoModerationTrigger {
            keyword_filter: None,
            presets: None,
            allow_list: None,
            mention_total_limit: None,
        },
        actions: vec![
            AutoModerationAction {
                kind: AutoModerationActionType::BlockMessage,
                metadata: Some(AutoModerationActionData {
                    channel_id: None,
                    duration_seconds: None,
                }),
            },
            AutoModerationAction {
                kind: AutoModerationActionType::SendAlertMessage,
                metadata: Some(AutoModerationActionData {
                    channel_id: Some(Id(1_015_680_953_001_189_488)),
                    duration_seconds: None,
                }),
            },
        ],
        enabled: true,
        exempt_roles: vec![],
        exempt_channels: vec![],
    }
}

fn flagged_words_rule() -> AutoModerationRule {
    AutoModerationRule {
        id: Id(1_015_703_489_575_403_601),
        guild_id: guild_id(),
        name: "Commonly Flagged Words".to_owned(),
        creator_id: Id(258_568_289_746_288_641),
        event_type: AutoModerationEvent::MessageSend,
        trigger_type: AutoModerationTriggerType::KeywordPreset,
        trigger_metadata: AutoModerationTrigger {
            keyword_filter: None,
            presets: Some(vec![
                KeywordPresetType::SexualContent,
                KeywordPresetType::Slurs,
            ]),
            allow_list: Some(vec!["bar".to_owned(), "foo".to_owned()]),
            mention_total_limit: None,
        },
        actions: vec![
            AutoModerationAction {
                kind: AutoModerationActionType::BlockMessage,
                metadata: Some(AutoModerationActionData {
                    channel_id: None,
                    duration_seconds: None,
                }),
            },
            AutoModerationAction {
                kind: AutoModerationActionType::SendAlertMessage,
                metadata: Some(AutoModerationActionData {
                    channel_id: Some(Id(1_015_680_953_001_189_488)),
                    duration_seconds: None,
                }),
            },
        ],
        enabled: true,
        exempt_roles: vec![],
        exempt_channels: vec![],
    }
}

fn disabled_rule() -> AutoModerationRule {
    AutoModerationRule {
        id: Id(1_015_741_735_722_287_134),
        guild_id: guild_id(),
        name: "Disabled".to_owned(),
        creator_id: Id(258_568_289_746_288_641),
        event_type: AutoModerationEvent::MessageSend,
        trigger_type: AutoModerationTriggerType::Keyword,
        trigger_metadata: AutoModerationTrigger {
            keyword_filter: Some(vec![
                "a".to_owned(),
                "*b".to_owned(),
                "c*".to_owned(),
                "*d*".to_owned(),
            ]),
            presets: None,
            allow_list: None,
            mention_total_limit: None,
        },
        actions: vec![AutoModerationAction {
            kind: AutoModerationActionType::BlockMessage,
            metadata: Some(AutoModerationActionData {
                channel_id: None,
                duration_seconds: None,
            }),
        }],
        enabled: false,
        exempt_roles: vec![Id(1_015_677_267_063_619_654)],
        exempt_channels: vec![Id(1_015_680_953_001_189_488)],
    }
}

#[tokio::test]
async fn auto_moderation_rules() {
    assert_eq!(
        CTX.auto_moderation_rules(guild_id()).await.unwrap(),
        vec![
            mention_spam_rule(),
            spam_rule(),
            flagged_words_rule(),
            disabled_rule()
        ]
    );
}

#[tokio::test]
async fn auto_moderation_rule() {
    let disabled_rule = disabled_rule();
    assert_eq!(
        CTX.auto_moderation_rule(guild_id(), disabled_rule.id)
            .await
            .unwrap(),
        disabled_rule
    );
}
