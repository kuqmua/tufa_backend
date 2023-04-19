// use crate::global_variables::runtime::config::CONFIG;
// use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
// use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
// use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsWrapperError;
// use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
// use impl_get_git_info::ImplGetGitInfoFromTufaCommon;
// use impl_get_source::ImplGetSourceFromTufaCommon;
// use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
// use init_error::InitErrorFromTufaCommon;
// use strum_macros::EnumIter;
// use tufa_common::common::where_was::WhereWas;
// use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
// use tufa_common::traits::get_source::GetSource;
// use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

// #[derive(Debug, EnumIter)]
// pub enum InitTablesEnum {
//     ProvidersLinkParts,
// }

// #[derive(
//     Debug,
//     InitErrorFromTufaCommon,
//     ImplErrorWithTracingFromTufaCommon,
//     ImplGetSourceFromTufaCommon,
//     ImplGetWhereWasOriginOrWrapperFromTufaCommon,
//     ImplGetGitInfoFromTufaCommon,
// )]
// pub struct InitTablesWrapperError {
//     source: InitTablesWrapperErrorEnum,
//     where_was: WhereWas,
// }

// #[derive(Debug, ImplGetWhereWasOriginOrWrapperFromTufaCommon, ImplGetSourceFromTufaCommon)]
// pub enum InitTablesWrapperErrorEnum {
//     ProvidersLinkPartsWrapper(InitDbsProvidersLinkPartsWrapperError),
// }

// impl InitTablesEnum {
//     pub async fn init(&self, should_trace: bool) -> Result<(), Box<InitTablesWrapperError>> {
//         match self {
//             InitTablesEnum::ProvidersLinkParts => {
//                 if let Err(e) = init_dbs_with_providers_link_parts(false).await {
//                     return Err(Box::new(
//                         InitTablesWrapperError::init_error_with_possible_trace(
//                             InitTablesWrapperErrorEnum::ProvidersLinkPartsWrapper(*e),
//                             WhereWas {
//                                 time: std::time::SystemTime::now()
//                                     .duration_since(std::time::UNIX_EPOCH)
//                                     .expect("cannot convert time to unix_epoch"),
//                                 file: String::from(file!()),
//                                 line: line!(),
//                                 column: column!(),
//                                 git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
//                             },
//                             &CONFIG.source_place_type,
//                             should_trace,
//                         ),
//                     ));
//                 }
//             }
//         }
//         Ok(())
//     }
// }
