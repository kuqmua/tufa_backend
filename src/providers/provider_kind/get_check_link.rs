use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::config::CONFIG;

impl ProviderKind {
    pub fn get_check_link(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => &CONFIG.providers_check_links.arxiv_link,
            ProviderKind::Biorxiv => &CONFIG.providers_check_links.biorxiv_link,
            ProviderKind::Github => &CONFIG.providers_check_links.github_link,
            ProviderKind::Medrxiv => &CONFIG.providers_check_links.medrxiv_link,
            ProviderKind::Twitter => &CONFIG.providers_check_links.twitter_link,
            ProviderKind::Reddit => &CONFIG.providers_check_links.reddit_link,
            ProviderKind::Habr => &CONFIG.providers_check_links.habr_link,
        }
    }
}