use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use futures::future::join_all;
use std::collections::HashMap;
use std::ops::Deref;
use tufa_common::common::where_was::WhereWas;
use tufa_common::server::mongo::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
use tufa_common::server::mongo::mongo_insert_docs_in_empty_collection::MongoInsertDocsInEmptyCollectionWrapperError;

#[derive(Debug)]
pub struct MongoInsertDataWrapperError {
    pub source: HashMap<ProviderKind, MongoInsertDocsInEmptyCollectionWrapperError>,
    where_was: WhereWas,
}

pub async fn mongo_insert_data(
    db_name_handle: &str,
    vec_of_link_parts_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), Box<MongoInsertDataWrapperError>> {
    let error_hashmap = join_all(vec_of_link_parts_hashmap.into_iter().map(
        |(pk, vec_of_link_parts)| async move {
            (
                pk,
                mongo_insert_docs_in_empty_collection(
                    MONGO_CLIENT_OPTIONS.deref().to_owned(),
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
    .collect::<HashMap<ProviderKind, MongoInsertDocsInEmptyCollectionWrapperError>>();
    if !error_hashmap.is_empty() {
        return Err(Box::new(MongoInsertDataWrapperError {
            source: error_hashmap,
            where_was: WhereWas {
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
                        crate::global_variables::compile_time::git_info::GIT_INFO.commit_unix_time,
                    ),
                    timezone: String::from(
                        crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                    ),
                    message: String::from(
                        crate::global_variables::compile_time::git_info::GIT_INFO.message,
                    ),
                },
            },
        }));
    }
    Ok(())
}
