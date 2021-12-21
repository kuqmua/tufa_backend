use procedural_macros_lib::EnumIntoArray;
use procedural_macros_lib::EnumVariantCount;

use strum_macros::Display;
use strum_macros::EnumIter;

use crate::traits::enum_extention::EnumExtenstion;

use convert_case::{Case, Casing};

use std::collections::HashMap;

use strum::IntoEnumIterator;

#[derive(
    EnumIntoArray,
    EnumVariantCount,
    EnumExtenstion,
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
pub enum EnvI64Var {
    CommonProvidersLinksLimit,
    LinksLimitForArxiv,
    LinksLimitForBiorxiv,
    LinksLimitForGithub,
    LinksLimitForHabr,
    LinksLimitForMedrxiv,
    LinksLimitForReddit,
    LinksLimitForTwitter,
}

impl EnvI64Var {
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
}
