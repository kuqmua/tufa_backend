#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableRandomizeOrderForProvidersLinkPartsForMongo {
    pub enable_randomize_order_for_arxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_biorxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_github_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_habr_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_medrxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_reddit_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_twitter_link_parts_for_mongo: bool,
}
