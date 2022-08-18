use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasTracing;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use itertools::Itertools;
#[derive(Debug)]
pub enum GetLinkPartsFromLocalJsonFileErrorEnum {
    TokioFsFileOpen {
        source: std::io::Error,
        where_was: WhereWas,
    },
    TokioIoAsyncReadExtReadToEnd {
        source: std::io::Error,
        where_was: WhereWas,
    },
    SerdeJsonFromSlice {
        source: serde_json::Error,
        where_was: WhereWas,
    },
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
    ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileErrorEnum>> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                Err(Box::new(
                    GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen {
                        source: e,
                        where_was,
                    },
                ))
            }
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd {
                            source: e,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        },
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice {
                            source: e,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        },
                    )),
                    Ok(file_content_as_struct) => {
                        let unique_vec: Vec<String> =
                            file_content_as_struct.data.into_iter().unique().collect();
                        let len = unique_vec.len();
                        let return_vec: Vec<String>;
                        //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
                        if self.is_links_limit_enabled() {
                            let limit = self.links_limit().try_into().unwrap(); //todo i64 type change?
                            if len > limit {
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
