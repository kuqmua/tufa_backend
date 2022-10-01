use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfo;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas;
use impl_get_source_for_enum_without_method::ImplGetSourceForEnumWithoutMethod;
use impl_get_source_for_struct_with_method::ImplGetSourceForStructWithMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use reqwest::Error;
use reqwest::StatusCode;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceForStructWithMethod,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas,
)]
pub struct NetCheckAvailabilityError {
    source: NetCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

use tufa_common::config::source_place_type::SourcePlaceType;
use tufa_common::helpers::git::git_info::GitInformation;
use tufa_common::traits::get_bunyan_with_additional_where_was::GetBunyanWithAdditionalWhereWas;
use tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;
use tufa_common::where_was::WhereWasOneOrMany;

impl NetCheckAvailabilityError {
    //GetBunyanWithAdditionalWhereWas<NetCheckAvailabilityErrorEnum> for
    fn get_bunyan_with_additional_where_was(
        &self,
        where_was: &WhereWas,
        source_place_type: &SourcePlaceType,
        git_info: &GitInformation,
    ) -> String {
        match source_place_type {
            SourcePlaceType::Source => {
                let second_arg = match self.get_where_was_one_or_many() {
                    WhereWasOneOrMany::One(where_was_with_addition) => {
                        where_was_with_addition.where_was.file_line_column()
                    }
                    WhereWasOneOrMany::Many(vec_where_was_with_addition) => {
                        let mut formatted_into_string_vec = vec_where_was_with_addition
                            .iter()
                            .map(|where_was_with_addition| {
                                format!(
                                    "{}, ",
                                    where_was_with_addition
                                        .get_file_line_column(source_place_type, git_info)
                                )
                            }) //maybe here use \n
                            .collect::<Vec<String>>()
                            .iter()
                            .fold(String::from(""), |mut acc, elem| {
                                acc.push_str(elem);
                                acc
                            });
                        if !formatted_into_string_vec.is_empty() {
                            formatted_into_string_vec.pop();
                            formatted_into_string_vec.pop();
                        }
                        format!("[{}]", formatted_into_string_vec)
                    }
                };
                format!("{} {}", where_was.file_line_column(), second_arg)
            }
            SourcePlaceType::Github => {
                let second_arg = match self.get_where_was_one_or_many() {
                    WhereWasOneOrMany::One(where_was_with_addition) => {
                        where_was_with_addition.where_was.file_line_column()
                    }
                    WhereWasOneOrMany::Many(vec_where_was_with_addition) => {
                        let mut formatted_into_string_vec = vec_where_was_with_addition
                            .iter()
                            .map(|where_was_with_addition| {
                                format!(
                                    "{}, ",
                                    where_was_with_addition
                                        .get_file_line_column(source_place_type, git_info)
                                )
                            }) //maybe here use \n
                            .collect::<Vec<String>>()
                            .iter()
                            .fold(String::from(""), |mut acc, elem| {
                                acc.push_str(elem);
                                acc
                            });
                        if !formatted_into_string_vec.is_empty() {
                            formatted_into_string_vec.pop();
                            formatted_into_string_vec.pop();
                        }
                        format!("[{}]", formatted_into_string_vec)
                    }
                };
                format!(
                    "{} {}",
                    where_was.github_file_line_column(git_info),
                    second_arg
                )
            }
            SourcePlaceType::None => match self.get_where_was_one_or_many() {
                WhereWasOneOrMany::One(where_was_with_addition) => {
                    where_was_with_addition.where_was.file_line_column()
                }
                WhereWasOneOrMany::Many(vec_where_was_with_addition) => {
                    let mut formatted_into_string_vec = vec_where_was_with_addition
                        .iter()
                        .map(|where_was_with_addition| {
                            format!(
                                "{}, ",
                                where_was_with_addition
                                    .get_file_line_column(source_place_type, git_info)
                            )
                        }) //maybe here use \n
                        .collect::<Vec<String>>()
                        .iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(elem);
                            acc
                        });
                    if !formatted_into_string_vec.is_empty() {
                        formatted_into_string_vec.pop();
                        formatted_into_string_vec.pop();
                    }
                    format!("[{}]", formatted_into_string_vec)
                }
            },
        }
    }
}

#[derive(Debug, GitInfo, ImplDisplayForSimpleErrorEnum, ImplGetSourceForEnumWithoutMethod)]
pub enum NetCheckAvailabilityErrorEnum {
    ReqwestGet(Error),
    Client(StatusCode),
    Server(StatusCode),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn net_check_availability(
    link: &str,
    should_trace: bool,
) -> Result<(), Box<NetCheckAvailabilityError>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(
            NetCheckAvailabilityError::init_error_with_possible_trace(
                NetCheckAvailabilityErrorEnum::ReqwestGet(e),
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(res) => {
            let status = res.status();
            if status.is_client_error() {
                return Err(Box::new(
                    NetCheckAvailabilityError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::Client(status),
                        WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            if status.is_server_error() {
                return Err(Box::new(
                    NetCheckAvailabilityError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::Server(status),
                        WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            Ok(())
        }
    }
}
