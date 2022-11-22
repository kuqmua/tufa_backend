use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::Database;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
)]
pub struct MongoCheckCollectionsIsNotEmptyWrapperError {
    source: HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceFromTufaCommon, ImplDisplayForError)]
pub enum CollectionCountDocumentsOrIsNotEmpty {
    CountDocumentsOrigin(Error),
    IsNotEmptyOrigin(u64),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_collections_is_not_empty(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    db: &Database,
    should_trace: bool,
) -> Result<(), Box<MongoCheckCollectionsIsNotEmptyWrapperError>> {
    let error_vec_count_documents =
        join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
            (
                *pk,
                db.collection::<Document>(&pk.get_db_tag())
                    .count_documents(None, None)
                    .await,
            )
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| match result {
            Err(e) => Some((
                pk,
                CollectionCountDocumentsOrIsNotEmpty::CountDocumentsOrigin(e),
            )),
            Ok(documents_number) => {
                if documents_number > 0 {
                    return Some((
                        pk,
                        CollectionCountDocumentsOrIsNotEmpty::IsNotEmptyOrigin(documents_number),
                    ));
                }
                None
            }
        })
        .collect::<HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>>();
    if !error_vec_count_documents.is_empty() {
        return Err(Box::new(
            MongoCheckCollectionsIsNotEmptyWrapperError::init_error_with_possible_trace(
                error_vec_count_documents,
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: tufa_common::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        ));
    }
    Ok(())
}
