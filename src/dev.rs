use serde::{Deserialize, Serialize};
use thiserror::Error;
use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::error_display::ToStringHandle;
use tufa_common::traits::error_log::ErrorLog;
use tufa_common::traits::get_code_occurence::GetCodeOccurenceAsString;
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
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for OneWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use tufa_common::traits::get_color::ErrorColorBold;
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(f, "{}", self.source)
    }
}

impl<ConfigGeneric> tufa_common::traits::error_display::ToStringHandle<ConfigGeneric>
    for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn to_string_handle(&self, config: &ConfigGeneric) -> String {
        format!(
            "{}\n{}",
            self.source.to_string_handle(config),
            self.get_code_occurence_as_string(config),
        )
    }
}

impl<ConfigGeneric> GetSourceAsString<ConfigGeneric> for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        self.source.get_source_as_string(config)
    }
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurenceOldWay for OneWrapperError {
    fn get_code_occurence_old_way(
        &self,
    ) -> &tufa_common::common::code_occurence::CodeOccurenceOldWay {
        &self.code_occurence
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

impl<ConfigGeneric> tufa_common::traits::get_source::GetSourceAsString<ConfigGeneric>
    for OneWrapperErrorEnum
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for OneWrapperErrorEnum
where
    ConfigGeneric: tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

pub fn one() -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::dev::three() {
        return Err(Box::new(OneWrapperError {
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
