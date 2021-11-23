use crate::providers::provider_kind_enum::ProviderKind;

const ARXIV_NAME_TO_CHECK: &str = "arxiv";
const BIORXIV_NAME_TO_CHECK: &str = "biorxiv";
const GITHUB_NAME_TO_CHECK: &str = "github";
const HABR_NAME_TO_CHECK: &str = "habr";
const MEDRXIV_NAME_TO_CHECK: &str = "medrxiv";
const REDDIT_NAME_TO_CHECK: &str = "reddit";
const TWITTER_NAME_TO_CHECK: &str = "twitter";

impl ProviderKind {
    pub fn get_string_name(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => ARXIV_NAME_TO_CHECK,
            ProviderKind::Biorxiv => BIORXIV_NAME_TO_CHECK,
            ProviderKind::Github => GITHUB_NAME_TO_CHECK,
            ProviderKind::Habr => HABR_NAME_TO_CHECK,
            ProviderKind::Medrxiv => MEDRXIV_NAME_TO_CHECK,
            ProviderKind::Reddit => REDDIT_NAME_TO_CHECK,
            ProviderKind::Twitter => TWITTER_NAME_TO_CHECK,
        }
    }
}