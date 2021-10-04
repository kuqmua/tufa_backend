#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableProvidersLinksLimit {
    pub enable_links_limit_for_arxiv: bool,
    pub enable_links_limit_for_biorxiv: bool,
    pub enable_links_limit_for_github: bool,
    pub enable_links_limit_for_habr: bool,
    pub enable_links_limit_for_medrxiv: bool,
    pub enable_links_limit_for_reddit: bool,
    pub enable_links_limit_for_twitter: bool,
}
