use crate::providers::provider_kind_enum::ProviderKind;

#[derive(Default, Debug, Clone, PartialEq)] //, serde_derive::Serialize, serde_derive::Deserialize
pub struct EnableInitializeMongoWithProvidersLinkParts {
    pub enable_initialize_mongo_with_arxiv_link_parts: bool,
    pub enable_initialize_mongo_with_biorxiv_link_parts: bool,
    pub enable_initialize_mongo_with_github_link_parts: bool,
    pub enable_initialize_mongo_with_habr_link_parts: bool,
    pub enable_initialize_mongo_with_medrxiv_link_parts: bool,
    pub enable_initialize_mongo_with_reddit_link_parts: bool,
    pub enable_initialize_mongo_with_twitter_link_parts: bool,
}

impl EnableInitializeMongoWithProvidersLinkParts {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_initialization_enabled(&self, provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => self.enable_initialize_mongo_with_arxiv_link_parts,
            ProviderKind::Biorxiv => self.enable_initialize_mongo_with_biorxiv_link_parts,
            ProviderKind::Github => self.enable_initialize_mongo_with_github_link_parts,
            ProviderKind::Habr => self.enable_initialize_mongo_with_habr_link_parts,
            ProviderKind::Medrxiv => self.enable_initialize_mongo_with_medrxiv_link_parts,
            ProviderKind::Reddit => self.enable_initialize_mongo_with_reddit_link_parts,
            ProviderKind::Twitter => self.enable_initialize_mongo_with_twitter_link_parts,
        }
    }
}
