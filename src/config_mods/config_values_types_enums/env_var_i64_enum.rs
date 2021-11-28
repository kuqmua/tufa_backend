use procedural_macros_lib::AllVariants;
use procedural_macros_lib::EnumVariantCount;

use strum_macros::EnumIter;

#[derive(
    AllVariants,
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
