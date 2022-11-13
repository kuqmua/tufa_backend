use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use futures::future::join_all;
use git_info::GitInfoFromTufaCommon;
use std::collections::HashMap;
use tufa_common::server::mongo::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
use tufa_common::server::mongo::mongo_insert_docs_in_empty_collection::MongoInsertDocsInEmptyCollectionError;

//
// use tufa_common::traits::git_info_trait::GitInfo;
use tufa_common::common::where_was::WhereWas;

#[derive(Debug, GitInfoFromTufaCommon)]
pub struct MongoInsertDataError {
    pub source: HashMap<ProviderKind, MongoInsertDocsInEmptyCollectionError>,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_insert_data(
    db_name_handle: &str,
    vec_of_link_parts_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), Box<MongoInsertDataError>> {
    let error_hashmap = join_all(vec_of_link_parts_hashmap.into_iter().map(
        |(pk, vec_of_link_parts)| async move {
            (
                pk,
                mongo_insert_docs_in_empty_collection(
                    {
                        use tufa_common::config_mods::traits::get_mongo_url_trait::GetMongoUrl;
                        CONFIG.get_mongo_url()
                    },
                    db_name_handle,
                    format!(
                        "{pk}{}",
                        CONFIG.mongo_providers_logs_db_collection_handle_second_part
                    ),
                    CONFIG
                        .mongo_providers_logs_db_collection_document_field_name_handle
                        .clone(),
                    vec_of_link_parts,
                    &CONFIG.source_place_type,
                    false,
                )
                .await,
            )
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((pk, *e));
        }
        None
    })
    .collect::<HashMap<ProviderKind, MongoInsertDocsInEmptyCollectionError>>();
    if !error_hashmap.is_empty() {
        return Err(Box::new(MongoInsertDataError {
            source: error_hashmap,
            where_was: WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                location: *core::panic::Location::caller(),
            },
        }));
    }
    Ok(())
}
