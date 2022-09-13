use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use tufa_common::traits::get_source::GetSource;
use tufa_common::where_was::WhereWas;
// use crate::traits::get_where_was::GetWhereWas;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
// use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
// use init_error_with_tracing::InitErrorWithTracing;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use itertools::Itertools;

#[derive(
    Debug,
    // ImplGetWhereWasForErrorStruct,
    ImplGetSourceForParentErrorStruct,
    ImplDisplayForErrorStruct,
    InitError,
    InitErrorWithTracingForOriginalErrorStruct,
)]
pub struct GetLinkPartsFromLocalJsonFileError {
    source: GetLinkPartsFromLocalJsonFileErrorEnum,
    where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for GetLinkPartsFromLocalJsonFileError
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}

#[derive(Debug, ImplGetSourceForSimpleErrorEnum, ImplDisplayForSimpleErrorEnum)]
pub enum GetLinkPartsFromLocalJsonFileErrorEnum {
    TokioFsFileOpen(std::io::Error),
    TokioIoAsyncReadExtReadToEnd(std::io::Error),
    SerdeJsonFromSlice(serde_json::Error),
}

impl ProviderKind {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn get_link_parts_from_local_json_file(
        self,
        should_trace: bool,
    ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileError>> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                match should_trace {
                    true => Err(Box::new(GetLinkPartsFromLocalJsonFileError::with_tracing(
                        GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen(e),
                        where_was,
                    ))),
                    false => Err(Box::new(GetLinkPartsFromLocalJsonFileError::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen(e),
                        where_was,
                    ))),
                }
            }
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    let where_was = WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    };
                    match should_trace {
                        true => {
                            return Err(Box::new(GetLinkPartsFromLocalJsonFileError::with_tracing(
                            GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd(e),
                            where_was,
                        )));
                        }
                        false => {
                            return Err(Box::new(GetLinkPartsFromLocalJsonFileError::new(
                            GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd(e),
                            where_was,
                        )));
                        }
                    }
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => {
                        let where_was = WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        };
                        match should_trace {
                            true => {
                                Err(Box::new(GetLinkPartsFromLocalJsonFileError::with_tracing(
                                    GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice(e),
                                    where_was,
                                )))
                            }
                            false => Err(Box::new(GetLinkPartsFromLocalJsonFileError::new(
                                GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice(e),
                                where_was,
                            ))),
                        }
                    }
                    Ok(file_content_as_struct) => {
                        let unique_vec: Vec<String> =
                            file_content_as_struct.data.into_iter().unique().collect();
                        let return_vec: Vec<String>;
                        //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
                        if CONFIG.is_links_limit_enabled_providers && self.is_links_limit_enabled()
                        {
                            let limit = self.links_limit();
                            if unique_vec.len() > limit {
                                return_vec = unique_vec
                                    .into_iter()
                                    .enumerate()
                                    .filter_map(|(index, element)| match index > limit {
                                        false => None,
                                        true => Some(element),
                                    })
                                    .collect::<Vec<String>>();
                            } else {
                                return_vec = unique_vec;
                            }
                        } else {
                            return_vec = unique_vec;
                        }
                        Ok(return_vec)
                    }
                }
            }
        }
    }
}
