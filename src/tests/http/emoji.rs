use crate::tests::{guild, CTX};

#[tokio::test]
async fn emojis() {
    let guild = guild();
    assert_eq!(CTX.emojis(guild.id).await.unwrap(), guild.emojis);
}

#[tokio::test]
async fn emoji() {
    let guild = guild();
    for emoji in guild.emojis {
        assert_eq!(CTX.emoji(guild.id, emoji.id.unwrap()).await.unwrap(), emoji);
    }
}
