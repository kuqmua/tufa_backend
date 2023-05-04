pub async fn init_mongo<'a>(
    providers_json_local_data_hashmap: std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoErrorNamed<'a>>>{
    match crate::mongo_integration::mongo_client_options_parse::mongo_client_options_parse().await {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoErrorNamed::ClientOptionsParse {
                client_options_parse: *e,
                code_occurence: tufa_common::code_occurence!()
            }
        )),
        Ok(client_options) => match crate::mongo_integration::mongo_client_with_options::mongo_client_with_options(client_options) {
            Err(e) => Err(Box::new(
                tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoErrorNamed::ClientWithOptions {
                    client_with_options: *e,
                    code_occurence: tufa_common::code_occurence!()
                }
            )),
            Ok(client) => {
                let db = client.database(&crate::global_variables::runtime::config::CONFIG.mongo_providers_link_parts_db_name);
                if let Err(e) = tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::mongo_check_collections_is_not_empty(
                    providers_json_local_data_hashmap.clone(),
                    &db,
                )
                .await
                {
                    return Err(Box::new(
                        tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoErrorNamed::CollectionIsNotEmpty {
                            collection_is_not_empty: *e,
                            code_occurence: tufa_common::code_occurence!()
                        }
                    ));
                }
                if let Err(e) =
                    crate::mongo_integration::mongo_insert_many::mongo_insert_many(providers_json_local_data_hashmap, db, false).await
                {
                    return Err(Box::new(
                        tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoErrorNamed::InsertMany {
                            insert_many: *e,
                            code_occurence: tufa_common::code_occurence!()
                        }
                    ));
                }
                Ok(())
            }
        },
    }
}
