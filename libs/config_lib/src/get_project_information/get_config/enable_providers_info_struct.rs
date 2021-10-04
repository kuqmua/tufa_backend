#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableProvidersInfo {
    pub enable_info_for_arxiv: bool,
    pub enable_info_for_biorxiv: bool,
    pub enable_info_for_github: bool,
    pub enable_info_for_habr: bool,
    pub enable_info_for_medrxiv: bool,
    pub enable_info_for_reddit: bool,
    pub enable_info_for_twitter: bool,
}
