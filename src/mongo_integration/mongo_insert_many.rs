
pub async fn mongo_insert_many<'a>(
    providers_json_local_data_hashmap: std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    db: mongodb::Database,
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorNamed<'a>>> {
    let error_vec_insert_many = futures::future::join_all(
        providers_json_local_data_hashmap.iter().map(
                |(pk, data_vec)|
                async {
                    (
                        pk.to_string(), 
                        db
                        .collection(&{
                            use tufa_common::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                            pk.get_db_tag()
                        })
                        .insert_many(
                            data_vec
                            .iter()
                            .map(|data|
                                mongodb::bson::doc! { &crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }
                            )
                            .collect::<Vec<mongodb::bson::Document>>(), 
                            None
                        ).await
                    )
                }
            )
        ).await
        .into_iter()
        .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((
                pk.to_string(), 
                tufa_common::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorUnnamed::InsertMany(
                    tufa_common::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyHandleErrorNamed::InsertMany { 
                        insert_many: e, 
                        code_occurence: tufa_common::code_occurence!()  
                    }
                )
            ));
        }
        None
    })
    .collect::<std::collections::HashMap<String, tufa_common::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorUnnamed>>();
    match error_vec_insert_many.is_empty() {
        true => Ok(()),
        false => Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_insert_many::MongoInsertManyErrorNamed::InsertMany { 
                insert_many: error_vec_insert_many, 
                code_occurence: tufa_common::code_occurence!() 
            },
        ))
    }
}
