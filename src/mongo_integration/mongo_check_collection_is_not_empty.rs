use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::Database;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceWithMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
)]
pub struct MongoCheckCollectionsIsNotEmptyWrapperError {
    source: HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromTufaCommon)]
pub enum CollectionCountDocumentsOrIsNotEmpty {
    CountDocuments(Error),
    IsNotEmpty(u64),
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
            Err(e) => Some((pk, CollectionCountDocumentsOrIsNotEmpty::CountDocuments(e))),
            Ok(documents_number) => {
                if documents_number > 0 {
                    return Some((
                        pk,
                        CollectionCountDocumentsOrIsNotEmpty::IsNotEmpty(documents_number),
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
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        ));
    }
    Ok(())
}
