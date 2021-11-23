use crate::providers::provider_kind_enum::ProviderKind;

use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
use crate::constants::project_constants::HABR_NAME_TO_CHECK;
use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;

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