use crate::providers::provider_kind_enum::ProviderKind;

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

impl EnableProviders {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_enabled(&self, provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => self.enable_arxiv,
            ProviderKind::Biorxiv => self.enable_biorxiv,
            ProviderKind::Github => self.enable_github,
            ProviderKind::Habr => self.enable_habr,
            ProviderKind::Medrxiv => self.enable_medrxiv,
            ProviderKind::Reddit => self.enable_reddit,
            ProviderKind::Twitter => self.enable_twitter,
        }
    }
}
