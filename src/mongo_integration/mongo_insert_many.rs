use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_methods::ProviderKindMethods;
use futures::future::join_all;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::Database;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
)]
pub struct MongoInsertManyOriginError {
    source: HashMap<ProviderKind, Error>,
    where_was: WhereWas,
}

pub async fn mongo_insert_many(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    db: Database,
    should_trace: bool,
) -> Result<(), Box<MongoInsertManyOriginError>> {
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
            MongoInsertManyOriginError::init_error_with_possible_trace(
                error_vec_insert_many,
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
    Ok(())
}
