pub async fn mongo_insert_data<'a>(
    db_name_handle: &'a str,
    vec_of_link_parts_hashmap: std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,//todo impl Display instead of ProviderKind
) -> Result<(), Box<tufa_common::server::mongo::mongo_insert_data::MongoInsertDataErrorNamed<'a>>> {
    let error_hashmap = futures::future::join_all(vec_of_link_parts_hashmap.into_iter().map(
        |(pk, vec_of_link_parts)| async move {
            (
                pk,
                tufa_common::server::mongo::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection(
                    {
                        use std::ops::Deref;
                        crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS.deref().to_owned()
                    },
                    db_name_handle,
                    format!(
                        "{pk}{}",
                        crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_collection_handle_second_part
                    ),
                    crate::global_variables::runtime::config::CONFIG
                        .mongo_providers_logs_db_collection_document_field_name_handle
                        .clone(),
                    vec_of_link_parts
                )
                .await,
            )
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((
                pk.to_string(), 
                tufa_common::server::mongo::mongo_insert_data::MongoInsertDataErrorUnnamed::MongoInsertDocsInEmptyCollection(*e)
            ));
        }
        None
    })
    .collect::<std::collections::HashMap<std::string::String, tufa_common::server::mongo::mongo_insert_data::MongoInsertDataErrorUnnamed>>();
    if !error_hashmap.is_empty() {
        return Err(Box::new(
            tufa_common::server::mongo::mongo_insert_data::MongoInsertDataErrorNamed::Errors {
                errors_hashmap: error_hashmap,
                code_occurence: tufa_common::code_occurence!()
            }
        ));
    }
    Ok(())
}