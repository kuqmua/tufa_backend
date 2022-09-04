use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::get_source::GetSource;
use crate::traits::get_where_was::GetWhereWas;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use init_error_with_tracing::InitErrorWithTracing;
use itertools::Itertools;

#[derive(
    Debug,
    ImplGetWhereWasForErrorStruct,
    ImplGetSourceForParentErrorStruct,
    ImplDisplayForErrorStruct,
    InitError,
    InitErrorWithTracing,
)]
pub struct GetLinkPartsFromLocalJsonFileError {
    source: GetLinkPartsFromLocalJsonFileErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug, ImplGetSourceForSimpleErrorEnum, ImplGetWhereWasForEnum, ImplDisplayForSimpleErrorEnum,
)]
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
                Err(Box::new(GetLinkPartsFromLocalJsonFileError {
                    source: GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen(e),
                    where_was,
                }))
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
                    return Err(Box::new(GetLinkPartsFromLocalJsonFileError {
                        source:
                            GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd(e),
                        where_was,
                    }));
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
                        Err(Box::new(GetLinkPartsFromLocalJsonFileError {
                            source: GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice(e),
                            where_was,
                        }))
                    }
                    Ok(file_content_as_struct) => {
                        let unique_vec: Vec<String> =
                            file_content_as_struct.data.into_iter().unique().collect();
                        let return_vec: Vec<String>;
                        //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
                        if self.is_links_limit_enabled() {
                            let limit = self.links_limit().try_into().unwrap(); //todo i64 type change?
                            if unique_vec.len() > limit {
                                return_vec = unique_vec[..limit].to_vec();
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
