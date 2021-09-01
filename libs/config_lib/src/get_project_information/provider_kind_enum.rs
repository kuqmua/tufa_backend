use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;
use procedural_macros_lib::EnumVariantCount;

//
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
                            //
#[derive(
    EnumVariantCount,
    // EnumIter
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
    // pub fn get_string_name(provider_kind_enum_type: Self) -> String {
    //     match provider_kind_enum_type {
    //         Self::Arxiv => ARXIV_NAME_TO_CHECK.to_string(),
    //         Self::Biorxiv => BIORXIV_NAME_TO_CHECK.to_string(),
    //         Self::Github => GITHUB_NAME_TO_CHECK.to_string(),
    //         Self::Habr => HABR_NAME_TO_CHECK.to_string(),
    //         Self::Medrxiv => MEDRXIV_NAME_TO_CHECK.to_string(),
    //         Self::Reddit => REDDIT_NAME_TO_CHECK.to_string(),
    //         Self::Twitter => TWITTER_NAME_TO_CHECK.to_string(),
    //     }
    // }
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
    // pub fn print_every_option() {
    //     for provider_kind in Self::iter() {
    //         println!("{:?}", provider_kind);
    //     }
    // }
}
// pub fn dosmthng() {
//     for provider_kind in ProviderKind::iter() {
//         println!("{:?}", provider_kind);
//     }
// }

#[derive(Debug, EnumIter)]
pub enum DDD {
    Ar,
    Bi,
    Gi,
}

pub fn dosmthng() {
    for provider_kind in DDD::iter() {
        println!("----------------{:?}", provider_kind);
    }
}
