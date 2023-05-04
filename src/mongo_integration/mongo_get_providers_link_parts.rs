pub async fn mongo_get_providers_link_parts<'a>(
) -> Result<std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed<'a>> {
    match mongodb::options::ClientOptions::parse({
        use tufa_common::traits::get_mongo_url::GetMongoUrl;
        crate::global_variables::runtime::config::CONFIG.get_mongo_url()
    }).await {
        Err(e) => Err(
            tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
                mongodb: e,
                code_occurence: tufa_common::code_occurence!(),
            }
        ),
        Ok(client_options) => match mongodb::Client::with_options(client_options) {
            Err(e) => Err(
                tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
                    mongodb: e,
                    code_occurence: tufa_common::code_occurence!(),
                }
            ),
            Ok(client) => {
                let db = client.database(&crate::global_variables::runtime::config::CONFIG.mongo_providers_link_parts_db_name);
                match db.list_collection_names(None).await {
                    Err(e) => Err(
                        tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
                            mongodb: e,
                            code_occurence: tufa_common::code_occurence!(),
                        }
                    ),
                    Ok(vec_collection_names) => {
                        let no_collection_error_hashmap = {
                            use tufa_common::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                            tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec()
                            .into_iter()
                            .filter_map(|pk| {
                                let collection_name = pk.get_mongo_log_collection_name();
                                if !vec_collection_names.contains(&collection_name) {
                                    return Some((pk.to_string(), collection_name));
                                }
                                None
                            })
                            .collect::<std::collections::HashMap<String, String>>()
                        };
                        if !no_collection_error_hashmap.is_empty() {
                            return Err(
                                tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::NoSuchCollections {
                                    no_such_collections: no_collection_error_hashmap,
                                    code_occurence: tufa_common::code_occurence!(),
                                }
                            );
                        }
                        let result_get_documents_hashmap =
                                futures::future::join_all({
                                    use tufa_common::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                                    tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec().iter().map(|pk| async {
                                        (
                                            *pk,
                                            crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector(
                                                db.collection::<mongodb::bson::Document>(&pk.get_mongo_log_collection_name()),
                                                &crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                                                tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind::get_mongo_provider_link_parts_aggregation(pk),
                                            )
                                            .await,
                                        )
                                    })
                                })
                            .await;
                        let mut success_hashmap: std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> =
                            std::collections::HashMap::with_capacity(result_get_documents_hashmap.len());
                        let mut error_hashmap: std::collections::HashMap<
                            String,
                            tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetDocumentsAsStringVectorErrorUnnamed,
                        > = std::collections::HashMap::new();
                        for (pk, result) in result_get_documents_hashmap.into_iter() {
                            match result {
                                Err(e) => {
                                    error_hashmap.insert(
                                        pk.to_string(), 
                                        tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetDocumentsAsStringVectorErrorUnnamed::MongoGetDocumentsAsStringVector(*e)
                                    );
                                }
                                Ok(vec) => {
                                    success_hashmap.insert(pk, vec);
                                }
                            }
                        }
                        if !error_hashmap.is_empty() {
                            return Err(
                                tufa_common::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::GetDocuments {
                                    get_documents: error_hashmap,
                                    code_occurence: tufa_common::code_occurence!(),
                                }
                            );
                        }
                        Ok(success_hashmap)
                    }
                }
            }
        },
    }
}
