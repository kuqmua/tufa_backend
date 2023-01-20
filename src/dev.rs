use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::code_path::CodePath;
use tufa_common::traits::error_log::ErrorLog;
use tufa_common::traits::fields::GetSourcePlaceType;
use tufa_common::traits::get_code_occurence::GetCodeOccurenceAsString;
use tufa_common::traits::separator_symbol::SeparatorSymbol;

pub fn dev() {
    let _f = one(true);
    if let Err(e) = _f {
        println!("{}", e);
    }
}

#[derive(Debug)]
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for OneWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}{} {}",
            self.source,
            config.symbol(),
            self.code_occurence
                .get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + self.code_occurence.time_file_line_column.time,
            )
            .with_timezone(&chrono::FixedOffset::east_opt(config.timezone).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
        )
    }
}

impl<ConfigGeneric> tufa_common::traits::get_source::GetSourceAsString<ConfigGeneric>
    for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetLogType
        + tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone,
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

impl<ConfigGeneric>
    tufa_common::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for OneWrapperError
where
    ConfigGeneric: tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::fields::GetLogType
        + tufa_common::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let vec = self.source.get_inner_source_and_code_occurence_vec(config);
        let mut sources_for_tracing: Vec<
            Vec<(
                tufa_common::common::source_and_code_occurence::Source,
                Vec<tufa_common::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::with_capacity(
            vec.iter()
                .map(|e| e.source.len())
                .collect::<Vec<usize>>()
                .iter()
                .sum(),
        );
        let vec_capacity = vec.len() + 1;
        let mut new_vec =
            vec.into_iter()
                .fold(Vec::with_capacity(vec_capacity), |mut acc, mut s| {
                    s.source.iter().for_each(|v| {
                        sources_for_tracing.push(v.clone());
                    });
                    s.add_one();
                    acc.push(s);
                    acc
                });
        new_vec.push(
            tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        new_vec
    }
}

#[derive(Debug)]
pub enum OneWrapperErrorEnum {
    ThreeWrapper(ThreeWrapperError),
}

impl std::fmt::Display for OneWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl<ConfigGeneric> tufa_common::traits::get_source::GetSourceAsString<ConfigGeneric>
    for OneWrapperErrorEnum
where
    ConfigGeneric: tufa_common::traits::fields::GetLogType
        + tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone,
{
    fn get_source_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_source_as_string(config),
        }
    }
}

impl<ConfigGeneric> GetCodeOccurenceAsString<ConfigGeneric> for OneWrapperErrorEnum
where
    ConfigGeneric:
        tufa_common::traits::fields::GetTimezone + tufa_common::traits::fields::GetSourcePlaceType,
{
    fn get_code_occurence_as_string(&self, config: &ConfigGeneric) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
}

impl<ConfigGeneric>
    tufa_common::traits::get_inner_source_and_code_occurence_vec::GetInnerSourceAndCodeOccurenceVec<
        ConfigGeneric,
    > for OneWrapperErrorEnum
where
    ConfigGeneric: tufa_common::traits::fields::GetTimezone
        + tufa_common::traits::fields::GetLogType
        + tufa_common::traits::fields::GetSourcePlaceType,
{
    fn get_inner_source_and_code_occurence_vec(
        &self,
        config: &ConfigGeneric,
    ) -> Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => {
                i.get_inner_source_and_code_occurence_vec(config)
            }
        }
    }
}

pub fn one(should_trace: bool) -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::dev::three(false) {
        let f = OneWrapperError {
            source: OneWrapperErrorEnum::ThreeWrapper(*e),
            code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: tufa_common::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        };
        f.log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
        return Err(Box::new(f));
    }
    Ok(())
}
