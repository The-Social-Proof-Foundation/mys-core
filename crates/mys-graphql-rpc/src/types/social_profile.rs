use async_graphql::SimpleObject;

/// Minimal representation of a profile from the social indexer.
#[derive(SimpleObject)]
pub struct SocialProfile {
    pub username: String,
}
