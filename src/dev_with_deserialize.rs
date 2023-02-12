use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum OneWrapperErrorEnum<'a> {
    ThreeWrapper(tufa_common::dev::ThreeWrapperError<'a>),
}

impl<'a> std::fmt::Display for OneWrapperErrorEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    tufa_common::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for OneWrapperErrorEnum<'a>
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use tufa_common::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime;
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a>
    tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<
        'a,
    > for OneWrapperErrorEnum<'a>
{
    fn to_string_without_config_lifetime(&self) -> String {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_without_config_lifetime(),
        }
    }
}
