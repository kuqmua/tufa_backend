use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::doc;
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
    ImplGetSourceWithoutMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon,
)]
pub struct MongoInsertManyError {
    source: HashMap<ProviderKind, Error>,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_insert_many(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    db: Database,
    should_trace: bool,
) -> Result<(), Box<MongoInsertManyError>> {
    let error_vec_insert_many = join_all(
        providers_json_local_data_hashmap.iter().map(
                |(pk, data_vec)|
                async {
                    let docs: Vec<Document> = data_vec
                    .iter()
                    .map(|data|
                        doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }
                    )
                    .collect();
                    (*pk, db.collection(&pk.get_db_tag()).insert_many(docs, None).await)
                }
            )
        ).await
        .into_iter()
        .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((pk, e));
        }
        None
    })
    .collect::<HashMap<ProviderKind, Error>>();
    if !error_vec_insert_many.is_empty() {
        return Err(Box::new(
            MongoInsertManyError::init_error_with_possible_trace(
                error_vec_insert_many,
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
