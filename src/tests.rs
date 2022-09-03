#![allow(clippy::unwrap_used)]

// Suppress `unused_crate_dependencies` false positives
use once_cell::sync::Lazy;

use crate::{model::Id, Context, ContextConfig};

static CTX: Lazy<Context> = Lazy::new(|| {
    Context::new(&ContextConfig {
        token: env!("TEST_BOT_TOKEN"),
    })
});

fn guild_id() -> Id {
    Id(env!("TEST_GUILD_ID").parse().unwrap())
}

/// Tests for models
mod model;

#[test]
fn context_new() {
    assert_eq!(
        Context::new(&ContextConfig { token: "foo" }).token,
        "Bot foo"
    );
}
