use serde::{Deserialize, Serialize};
use thiserror::Error;
// use tufa_common::dev::ThreeWrapperError;
use tufa_common::dev::EightOriginError;
// use tufa_common::traits::error_logs_logic::error_log::{ErrorLog, ErrorLogLifetime};
use tufa_common::traits::error_logs_logic::error_log::ErrorLogLifetime;
use tufa_common::traits::error_logs_logic::origin_to_string_with_config::OriginToStringWithConfigLifetime;
use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;

pub fn dev() {
    let _f = one();
    if let Err(e) = _f {
        println!("{}", e);
        e.error_log_lifetime(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
    }
}
//from implementation was not generated by thiserror with 'a lifetime https://github.com/dtolnay/thiserror/issues/68

#[derive(Debug, Error, Serialize)]
pub enum OneWrapperError<'a> {
    Something {
        inner_error: OneWrapperErrorEnum<'a>,
        code_occurence: tufa_common::common::code_occurence::CodeOccurenceLifetime<'a>,
    },
}
//cannot make it with generics
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
                code_occurence,
            } => inner_error.to_string_with_config_lifetime(config),
        }
    }
}

impl<'a> tufa_common::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for OneWrapperError<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
        match self {
            OneWrapperError::Something { inner_error, code_occurence } => inner_error.to_string_without_config_lifetime(),
        }
    }
}

impl<'a> tufa_common::traits::get_code_occurence::GetCodeOccurenceLifetime<'a>
    for OneWrapperError<'a>
{
    fn get_code_occurence_lifetime(
        &self,
    ) -> &tufa_common::common::code_occurence::CodeOccurenceLifetime {
        match self {
            OneWrapperError::Something {
                inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

// #[derive(Debug, Error, Serialize, Deserialize)]
// pub enum OneWrapperErrorWithSerializeDeserialize<'a> {
//     Something {
//         inner_error: OneWrapperErrorEnumWithSerializeDeserialize<'a>,
//         code_occurence:
//             tufa_common::common::code_occurence::CodeOccurenceWithSerializeDeserialize<'a>,
//     },
// }

// impl<'a, ConfigGeneric>
//     tufa_common::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfigLifetime<
//         'a,
//         ConfigGeneric,
//     > for OneWrapperErrorWithSerializeDeserialize<'a>
// where
//     ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
//         + tufa_common::traits::fields::GetTimezone
//         + tufa_common::traits::get_server_address::GetServerAddress,
// {
//     fn source_to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
//         use tufa_common::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime;
//         match self {
//             OneWrapperErrorWithSerializeDeserialize::Something {
//                 inner_error,
//                 code_occurence,
//             } => inner_error.to_string_with_config_lifetime(config),
//         }
//     }
// }

// impl<'a> tufa_common::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for OneWrapperErrorWithSerializeDeserialize<'a> {
//     fn source_to_string_without_config_lifetime(&self) -> String {
//         use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
//         match self {
//             OneWrapperErrorWithSerializeDeserialize::Something { inner_error, code_occurence } => inner_error.to_string_without_config_lifetime(),
//         }
//     }
// }

// impl<'a> tufa_common::traits::get_code_occurence::GetCodeOccurenceLifetime<'a>
//     for OneWrapperErrorWithSerializeDeserialize<'a>
// {
//     fn get_code_occurence_lifetime(
//         &self,
//     ) -> &tufa_common::common::code_occurence::CodeOccurenceLifetime {
//         match self {
//             OneWrapperErrorWithSerializeDeserialize::Something {
//                 inner_error,
//                 code_occurence,
//             } => code_occurence,
//         }
//     }
// }

#[derive(Debug, Error, Serialize)]
pub enum OneWrapperErrorEnum<'a> {
    // ThreeWrapper(ThreeWrapperError),
    EightWrapper(tufa_common::dev::EightOriginError<'a>),
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
        use tufa_common::traits::error_logs_logic::origin_to_string_with_config::OriginToStringWithConfigLifetime; //remove later, it was only for lifetimes test
        match self {
            // OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_with_config(config),
            OneWrapperErrorEnum::EightWrapper(i) => i.origin_to_string_with_config_lifetime(config),
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
            // OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_without_config(),
            OneWrapperErrorEnum::EightWrapper(i) => i.to_string_without_config_lifetime(),
        }
    }
}

// #[derive(Debug, Error, Serialize, Deserialize)]
// pub enum OneWrapperErrorEnumWithSerializeDeserialize<'a> {
//     // ThreeWrapper(ThreeWrapperError),
//     EightWrapper(tufa_common::dev::EightOriginErrorWithSerializeDeserialize<'a>),
// }

// impl<'a> std::fmt::Display for OneWrapperErrorEnumWithSerializeDeserialize<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime;
//         write!(f, "{}", self.to_string_without_config_lifetime())
//     }
// }

// impl<'a, ConfigGeneric>
//     tufa_common::traits::error_logs_logic::to_string_with_config::ToStringWithConfigLifetime<
//         'a,
//         ConfigGeneric,
//     > for OneWrapperErrorEnumWithSerializeDeserialize<'a>
// where
//     ConfigGeneric: tufa_common::traits::fields::GetSourcePlaceType
//         + tufa_common::traits::fields::GetTimezone
//         + tufa_common::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_lifetime(&self, config: &ConfigGeneric) -> String {
//         use tufa_common::traits::error_logs_logic::origin_to_string_with_config::OriginToStringWithConfigLifetime; //remove later, it was only for lifetimes test
//         match self {
//             // OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_with_config(config),
//             OneWrapperErrorEnumWithSerializeDeserialize::EightWrapper(i) => {
//                 i.origin_to_string_with_config_lifetime(config)
//             }
//         }
//     }
// }

// impl<'a>
//     tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetime<
//         'a,
//     > for OneWrapperErrorEnumWithSerializeDeserialize<'a>
// {
//     fn to_string_without_config_lifetime(&self) -> String {
//         match self {
//             // OneWrapperErrorEnum::ThreeWrapper(i) => i.to_string_without_config(),
//             OneWrapperErrorEnumWithSerializeDeserialize::EightWrapper(i) => {
//                 i.to_string_without_config_lifetime()
//             }
//         }
//     }
// }

pub fn one<'a>() -> Result<(), Box<OneWrapperError<'a>>> {
    // if let Err(e) = tufa_common::dev::three() {
    if let Err(e) = tufa_common::dev::eight() {
        let g = tufa_common::common::code_occurence::CodeOccurenceLifetime::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            // once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
            file!(),
            line!(),
            column!(),
        );
        println!(
            "1 {}",
            std::mem::size_of::<tufa_common::common::code_occurence::CodeOccurenceLifetime>()
        );
        println!("2 {}", std::mem::size_of_val(&g));
        let f = OneWrapperError::Something {
            // source: OneWrapperErrorEnum::ThreeWrapper(*e),
            inner_error: OneWrapperErrorEnum::EightWrapper(*e),
            code_occurence: g,
        };
        println!("3 {}", std::mem::size_of::<OneWrapperError>());
        println!("4 {}", std::mem::size_of_val(&f));
        return Err(Box::new(f));
    }
    Ok(())
}
