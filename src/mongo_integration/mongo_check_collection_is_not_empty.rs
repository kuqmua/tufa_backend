pub async fn mongo_check_collections_is_not_empty<'a>(
    providers_json_local_data_hashmap: std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    db: &mongodb::Database,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed<'a>>>{
    let error_vec_count_documents =
        futures::future::join_all(providers_json_local_data_hashmap.keys().map(|pk| async move {
            (
                *pk,
                db.collection::<mongodb::bson::Document>(&format!("{pk}"))
                    .count_documents(None, None)
                    .await,
            )
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| match result {
            Err(e) => Some((
                pk.to_string(),
                tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed::CountDocumentsOrigin(
                    tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed::CountDocuments { 
                        error: e, 
                        code_occurence: tufa_common::code_occurence!(), 
                    }
                ),
            )),
            Ok(documents_number) => {
                if documents_number > 0 {
                    return Some((
                        pk.to_string(),
                        tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed::IsNotEmptyOrigin(
                            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed::IsNotEmptyOrigin { 
                                error: documents_number, 
                                code_occurence: tufa_common::code_occurence!()
                            }
                        )
                    ));
                }
                None
            }
        })
        .collect::<std::collections::HashMap<String, tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed>>();
    if !error_vec_count_documents.is_empty() {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed::Mongo {        
                inner_errors: error_vec_count_documents, 
                code_occurence: tufa_common::code_occurence!()
            }
        ));
    }
    Ok(())
}
