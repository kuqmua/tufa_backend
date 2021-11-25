use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

impl ProviderKind {
    pub fn is_prints_enabled(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.enable_providers_prints.enable_prints_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers_prints.enable_prints_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers_prints.enable_prints_github,
            ProviderKind::Habr => CONFIG.enable_providers_prints.enable_prints_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers_prints.enable_prints_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers_prints.enable_prints_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers_prints.enable_prints_twitter,
        }
    }
}