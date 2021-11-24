use strum_macros::EnumIter;

use mongodb::bson::doc;

use crate::constants::project_constants::ARXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::BIORXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::GITHUB_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::HABR_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::MEDRXIV_PROVIDER_ITEM_HANDLE;
use crate::constants::project_constants::TWITTER_PROVIDER_ITEM_HANDLE;

use procedural_macros_lib::EnumVariantCount;

#[derive(Debug)]
pub struct RemoveDirError {
    pub error: std::io::Error,
}

#[derive(Debug)]
pub enum CleanLogsDirError {
    PathIsNotDir { path: String },
    CannotRemoveDir { error: RemoveDirError },
}
impl From<String> for CleanLogsDirError {
    fn from(e: String) -> Self {
        CleanLogsDirError::PathIsNotDir { path: e }
    }
}
impl From<std::io::Error> for CleanLogsDirError {
    fn from(e: std::io::Error) -> Self {
        CleanLogsDirError::CannotRemoveDir {
            error: RemoveDirError { error: e },
        }
    }
}

#[derive(EnumVariantCount, EnumIter, Clone, Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Hash, Copy)]
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
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_item_handle(provider_kind: ProviderKind) -> Option<&'static str> {
        match provider_kind {
            ProviderKind::Arxiv => Some(ARXIV_PROVIDER_ITEM_HANDLE),
            ProviderKind::Biorxiv => Some(BIORXIV_PROVIDER_ITEM_HANDLE),
            ProviderKind::Github => Some(GITHUB_PROVIDER_ITEM_HANDLE),
            ProviderKind::Habr => Some(HABR_PROVIDER_ITEM_HANDLE),
            ProviderKind::Medrxiv => Some(MEDRXIV_PROVIDER_ITEM_HANDLE),
            ProviderKind::Reddit => None,
            ProviderKind::Twitter => Some(TWITTER_PROVIDER_ITEM_HANDLE),
        }
    }
}
