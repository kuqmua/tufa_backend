#[derive(Default, Debug, Clone, PartialEq)] //, serde_derive::Serialize, serde_derive::Deserialize
pub struct EnablePartialSuccessProvidersPrints {
    pub enable_partial_success_prints_for_arxiv: bool,
    pub enable_partial_success_prints_for_biorxiv: bool,
    pub enable_partial_success_prints_for_github: bool,
    pub enable_partial_success_prints_for_habr: bool,
    pub enable_partial_success_prints_for_medrxiv: bool,
    pub enable_partial_success_prints_for_reddit: bool,
    pub enable_partial_success_prints_for_twitter: bool,
}
