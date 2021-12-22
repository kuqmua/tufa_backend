use strum_macros::Display;
use strum_macros::EnumIter;

use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use convert_case::{Case, Casing};

use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods;
use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;

use dotenv::dotenv;

use crate::constants::project_constants::ENV_FILE_NAME;

#[derive(
    EnumExtenstion,
    EnvVarTypedTrait,
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
