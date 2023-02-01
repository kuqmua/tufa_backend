use serde::{Deserialize, Serialize};
use thiserror::Error;
use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::to_string_with_config::ToStringWithConfig;
use tufa_common::traits::error_log::ErrorLog;
use tufa_common::traits::get_code_occurence::GetCodeOccurence;
use tufa_common::traits::get_source::GetErrorWrapperSourceAsSting;
use tufa_common::traits::to_string_without_config::ToStringWithoutConfig;
use tufa_common::traits::to_string_with_config::SourceToStringWithConfig;
use tufa_common::traits::to_string_without_config::SourceToStringWithoutConfig;

pub fn dev() {
    let _f = one();
    if let Err(e) = _f {
        println!("{}", e);
        e.error_log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum OneWrapperError {
    // #[error("{source}\n{code_occurence}")]
    Something {
        source: OneWrapperErrorEnum,
        code_occurence: tufa_common::common::code_occurence::CodeOccurence,
    },
}

//cannot make it with generics
impl std::fmt::Display for OneWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string_without_config())
    }
}

impl<ConfigGeneric> tufa_common::traits::to_string_with_config::ToStringWithConfig<ConfigGeneric> for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}\n{}",
            self.source_to_string_with_config(config),//- difference origin\wrapper, for origin source_to_string_with_config can be not implemented 
            self.get_code_occurence().to_string_with_config(config),
        )
    }
}

impl tufa_common::traits::to_string_without_config::ToStringWithoutConfig for OneWrapperError
{
    fn to_string_without_config(&self) -> String {
        format!(
            "{}\n{}",
            self.source_to_string_without_config(),
            self.get_code_occurence()
        )
    }
}

impl<ConfigGeneric> tufa_common::traits::to_string_with_config::SourceToStringWithConfig<ConfigGeneric> for OneWrapperError 
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperError::Something { source, code_occurence } => source.to_string_with_config(config),
        }
    }
}

impl tufa_common::traits::to_string_without_config::SourceToStringWithoutConfig for OneWrapperError {
    fn source_to_string_without_config(&self) -> String {
        match self {
            OneWrapperError::Something { source, code_occurence } => source.to_string_without_config(),
        }
    }
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurence for OneWrapperError {
    fn get_code_occurence(
        &self,
    ) -> &tufa_common::common::code_occurence::CodeOccurence {
        match self {
            OneWrapperError::Something {
                source,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum OneWrapperErrorEnum {
    // #[error("{0}")]
    ThreeWrapper(ThreeWrapperError),
}

//
impl std::fmt::Display for OneWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string_without_config())
    }
}

impl<ConfigGeneric> tufa_common::traits::to_string_with_config::ToStringWithConfig<ConfigGeneric> for OneWrapperErrorEnum 
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_with_config(config),
        }
    }
}

impl tufa_common::traits::to_string_without_config::ToStringWithoutConfig for OneWrapperErrorEnum {
    fn to_string_without_config(&self) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_without_config(),
        }
    }
}
//

pub fn one() -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::dev::three() {
        return Err(Box::new(OneWrapperError::Something { 
            source: OneWrapperErrorEnum::ThreeWrapper(*e), 
            code_occurence: tufa_common::common::code_occurence::CodeOccurence::new(
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
        }));
    }
    Ok(())
}
