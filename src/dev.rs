use serde::{Deserialize, Serialize};
use thiserror::Error;
use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::error_display::ToStringHandle;
use tufa_common::traits::error_log::ErrorLog;
use tufa_common::traits::get_code_occurence::GetCodeOccurenceOldWay;
use tufa_common::traits::error_display::ToStringHandleCodeOccurence;
use tufa_common::traits::get_source::GetSourceAsString;

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
        code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay,
    },
}

impl<ConfigGeneric> tufa_common::traits::error_display::ToStringHandle<ConfigGeneric>
    for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperError::Something {
                source,
                code_occurence,
            } => format!(
                "{}\n{}",
                source.to_string_handle(config),
                self.get_code_occurence_old_way().to_string_handle_code_occurence(config),
            ),
        }
    }
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurenceOldWay for OneWrapperError {
    fn get_code_occurence_old_way(
        &self,
    ) -> &tufa_common::common::code_occurence::CodeOccurenceOldWay {
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

impl<ConfigGeneric> tufa_common::traits::error_display::ToStringHandle<ConfigGeneric>
    for OneWrapperErrorEnum
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_handle(config),
        }
    }
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurenceOldWay for OneWrapperErrorEnum
{
    fn get_code_occurence_old_way(&self) -> &tufa_common::common::code_occurence::CodeOccurenceOldWay {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_code_occurence_old_way(),
        }
    }
}

pub fn one() -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::dev::three() {
        return Err(Box::new(OneWrapperError::Something { 
            source: OneWrapperErrorEnum::ThreeWrapper(*e), 
            code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay::new(
                once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                    String::from(file!()),
                    line!(),
                    column!(),
                )
        }));
    }
    Ok(())
}
