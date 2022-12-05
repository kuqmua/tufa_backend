use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;

use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use itertools::Itertools;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

#[derive(
    Debug,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplDisplayForError,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
)]
pub struct GetLinkPartsFromLocalJsonFileWrapperError {
    source: GetLinkPartsFromLocalJsonFileOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    ImplGetSourceFromTufaCommon,
    ImplDisplayForError,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
)]
pub enum GetLinkPartsFromLocalJsonFileOriginErrorEnum {
    TokioFsFileOpenOrigin(std::io::Error),
    TokioIoAsyncReadExtReadToEndOrigin(std::io::Error),
    SerdeJsonFromSliceOrigin(serde_json::Error),
}

impl ProviderKind {
    pub async fn get_link_parts_from_local_json_file(
        self,
        should_trace: bool,
    ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileWrapperError>> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err(Box::new(
                GetLinkPartsFromLocalJsonFileWrapperError::init_error_with_possible_trace(
                    GetLinkPartsFromLocalJsonFileOriginErrorEnum::TokioFsFileOpenOrigin(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                    }, //
                    &CONFIG.source_place_type,
                    should_trace,
                ),
            )),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileWrapperError::init_error_with_possible_trace(
                            GetLinkPartsFromLocalJsonFileOriginErrorEnum::TokioIoAsyncReadExtReadToEndOrigin(e),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                            },
                            &CONFIG.source_place_type,
                            should_trace,
                        ),
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileWrapperError::init_error_with_possible_trace(
                            GetLinkPartsFromLocalJsonFileOriginErrorEnum::SerdeJsonFromSliceOrigin(
                                e,
                            ),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                file: String::from(file!()),
                                line: line!(),
                                column: column!(),
                                git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                            },
                            &CONFIG.source_place_type,
                            should_trace,
                        ),
                    )),
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
