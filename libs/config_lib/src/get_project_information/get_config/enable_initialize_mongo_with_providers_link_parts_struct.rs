#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableInitializeMongoWithProvidersLinkParts {
    pub enable_initialize_mongo_with_arxiv_link_parts: bool,
    pub enable_initialize_mongo_with_biorxiv_link_parts: bool,
    pub enable_initialize_mongo_with_github_link_parts: bool,
    pub enable_initialize_mongo_with_habr_link_parts: bool,
    pub enable_initialize_mongo_with_medrxiv_link_parts: bool,
    pub enable_initialize_mongo_with_reddit_link_parts: bool,
    pub enable_initialize_mongo_with_twitter_link_parts: bool,
}
