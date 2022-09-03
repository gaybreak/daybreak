#![allow(clippy::unwrap_used)]

// Suppress `unused_crate_dependencies` false positives
use once_cell::sync::Lazy;
use time::OffsetDateTime;

use crate::{
    model::{
        permission::{self, Permissions},
        Id,
    },
    Context, ContextConfig,
};

static CTX: Lazy<Context> = Lazy::new(|| {
    Context::new(&ContextConfig {
        token: env!("TEST_BOT_TOKEN"),
    })
});
const TEST_GUILD_ID: Id = Id(903367565349384202);

#[test]
fn context_new() {
    assert_eq!(
        Context::new(&ContextConfig { token: "foo" }).token,
        "Bot foo"
    )
}

#[test]
fn id_timestamp() {
    assert_eq!(
        Id(258_568_289_746_288_641).timestamp().unwrap(),
        OffsetDateTime::from_unix_timestamp(1_481_717_884).unwrap()
    );
}

#[test]
fn permissions_pretty_string() {
    assert_eq!(
        permission::to_pretty_string(Permissions::CreateInstantInvite | Permissions::KickMembers),
        "- Create Invite\n- Kick Members"
    );
}
