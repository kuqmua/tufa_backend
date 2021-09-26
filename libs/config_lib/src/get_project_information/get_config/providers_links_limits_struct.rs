#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ProvidersLinksLimits {
    pub links_limit_for_arxiv: i64,
    pub links_limit_for_biorxiv: i64,
    pub links_limit_for_github: i64,
    pub links_limit_for_habr: i64,
    pub links_limit_for_medrxiv: i64,
    pub links_limit_for_reddit: i64,
    pub links_limit_for_twitter: i64,
}
