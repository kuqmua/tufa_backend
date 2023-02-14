use tufa_common::traits::error_logs_logic::error_log::ErrorLogLifetime;

pub fn dev() {
    let _f = one();
    if let Err(e) = _f {
        //todo - this actually must be a config struct function, not an error - config.error_log_lifetime(e)
        e.error_log_lifetime(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
    }
}
//from implementation was not generated by thiserror with 'a lifetime https://github.com/dtolnay/thiserror/issues/68

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum OneWrapperError<'a> {
    Something {
        inner_error: crate::dev::OneWrapperErrorEnum<'a>,
        code_occurence: tufa_common::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}

impl<'a> std::fmt::Display for OneWrapperError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        write!(f, "{}", self.to_string_without_config_lifetime())
    }
}

impl<'a, ConfigGeneric>
    tufa_common::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
        'a,
        ConfigGeneric,
    > for OneWrapperError<'a>
where
    ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
        use tufa_common::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime;
        match self {
            OneWrapperError::Something {
                inner_error,
                code_occurence: _code_occurence,
            } => inner_error.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a> tufa_common::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for OneWrapperError<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        match self {
            OneWrapperError::Something { inner_error, code_occurence: _code_occurence } => inner_error.to_string_without_config_lifetime(),
        }
    }
}

impl<'a> tufa_common::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for OneWrapperError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &tufa_common::common::code_occurence::CodeOccurenceLifetime {
        match self {
            OneWrapperError::Something {
                inner_error: _inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
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
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_without_config_lifetime(),
        }
    }
}

pub fn one<'a>() -> Result<(), Box<OneWrapperError<'a>>> {
    if let Err(e) = tufa_common::dev::three() {
        return Err(Box::new(OneWrapperError::Something {
            inner_error: crate::dev::OneWrapperErrorEnum::ThreeWrapper(*e),
            code_occurence: tufa_common::code_occurence!(),
        }));
    }
    Ok(())
}
