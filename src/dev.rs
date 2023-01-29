use serde::{Deserialize, Serialize};
use thiserror::Error;
use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::error_to_string_with_config::ErrorToStringWithConfig;
use tufa_common::traits::error_log::ErrorLog;
use tufa_common::traits::get_code_occurence::GetCodeOccurence;
use tufa_common::traits::error_display::ToStringHandleCodeOccurence;

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
    #[error("{source}\n{code_occurence}")]
    Something {
        source: OneWrapperErrorEnum,
        code_occurence: tufa_common::common::code_occurence::CodeOccurence,
    },
}

impl<ConfigGeneric> tufa_common::traits::error_to_string_with_config::ErrorToStringWithConfig<ConfigGeneric>
    for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn error_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperError::Something {
                source,
                code_occurence,
            } => format!(
                "{}\n{}",
                source.error_to_string_with_config(config),
                self.get_code_occurence().to_string_handle_code_occurence(config),
            ),
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
    #[error("{0}")]
    ThreeWrapper(ThreeWrapperError),
}

impl<ConfigGeneric> tufa_common::traits::error_to_string_with_config::ErrorToStringWithConfig<ConfigGeneric>
    for OneWrapperErrorEnum
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn error_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.error_to_string_with_config(config),
        }
    }
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurence for OneWrapperErrorEnum
{
    fn get_code_occurence(&self) -> &tufa_common::common::code_occurence::CodeOccurence {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_code_occurence(),
        }
    }
}

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
