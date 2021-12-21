use strum_macros::EnumIter;

use mongodb::bson::doc;

use convert_case::{Case, Casing};

use std::collections::HashMap;

use strum_macros::Display;
use strum::IntoEnumIterator;

use procedural_macros_lib::EnumIntoArray;
use procedural_macros_lib::EnumVariantCount;

use crate::traits::enum_extention::EnumExtenstion;

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

#[derive(
    EnumExtenstion,
    EnumIntoArray,
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Display,
)]
#[strum(serialize_all = "snake_case")]
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
}
