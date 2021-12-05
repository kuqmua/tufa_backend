#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableProviders {
    pub enable_arxiv: bool,
    pub enable_biorxiv: bool,
    pub enable_github: bool,
    pub enable_habr: bool,
    pub enable_medrxiv: bool,
    pub enable_reddit: bool,
    pub enable_twitter: bool,
}
