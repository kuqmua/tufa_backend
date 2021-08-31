use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;
use procedural_macros_lib::EnumVariantCount;

#[derive(
    EnumVariantCount,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}

impl ProviderKind {
    pub fn get_string_name(provider_kind_enum_type: ProviderKind) -> String {
        match provider_kind_enum_type {
            ProviderKind::Arxiv => ARXIV_NAME_TO_CHECK.to_owned(),
            ProviderKind::Biorxiv => BIORXIV_NAME_TO_CHECK.to_owned(),
            ProviderKind::Github => GITHUB_NAME_TO_CHECK.to_owned(),
            ProviderKind::Habr => HABR_NAME_TO_CHECK.to_owned(),
            ProviderKind::Medrxiv => MEDRXIV_NAME_TO_CHECK.to_owned(),
            ProviderKind::Reddit => REDDIT_NAME_TO_CHECK.to_owned(),
            ProviderKind::Twitter => TWITTER_NAME_TO_CHECK.to_owned(),
        }
    }
    pub fn get_length() -> usize {
        PROVIDER_KIND_ENUM_LENGTH
    }
}
