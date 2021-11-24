use mongodb::{Client, options::ClientOptions, bson::{doc, Document}};

use crate::{config_mods::config::CONFIG, mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector};

use crate::{mongo_integration::mongo_get_db_url::mongo_get_db_url, prints::{print_colorful_message::print_colorful_message, print_type_enum::PrintType}, providers::provider_kind_enum::ProviderKind};

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_provider_link_parts_as_bson_string(
        provider_kind: ProviderKind,
    ) -> Result<Option<Vec<String>>, mongodb::error::Error> {
        //todo maybe option vec string
        let client_options = ClientOptions::parse(mongo_get_db_url()).await?;
        let client = Client::with_options(client_options)?;
        //declare db name. there is no create db method in mongo
        let db = client.database(&CONFIG.mongo_params.providers_db_name_handle);
        let mut needed_db_collection: Option<String> = None;
        for collection_name in db.list_collection_names(None).await? {
            if collection_name == *ProviderKind::get_mongo_log_collection_name(provider_kind) {
                needed_db_collection = Some(collection_name);
            }
        }
        if let Some(collection_name) = needed_db_collection {
            let collection = db.collection(&collection_name);
            let documents_number = collection.count_documents(None, None).await?;
            if documents_number > 0 {
                //rewrite as PrintType::Info or something
                print_colorful_message(
                    None,
                    PrintType::Success,
                    file!().to_string(),
                    line!().to_string(),
                    format!("collection.count_documents {}", documents_number),
                );
                let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<
                    Document,
                >;
                if CONFIG.params.enable_provider_links_limit {
                    if CONFIG.params.enable_common_providers_links_limit {
                        if CONFIG
                            .params
                            .enable_randomize_order_for_providers_link_parts_for_mongo
                        {
                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(
                                doc! { "$sample" : {"size": CONFIG.params.common_providers_links_limit }},
                            );
                        } else {
                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(
                                doc! { "$limit" :  CONFIG.params.common_providers_links_limit },
                            );
                        }
                    } else {
                        option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                            ProviderKind::get_mongo_doc_randomization_aggregation(provider_kind);
                    }
                } else {
                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                }
                // let aggregation_stage_1_get_docs_in_random_order_with_limit =
                //     doc! { "$sample" : {"size": 5 }};
                // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
                let vec_of_strings = mongo_get_documents_as_string_vector(
                    collection,
                    &CONFIG
                        .mongo_params
                        .providers_db_collection_document_field_name_handle,
                    option_aggregation_stage_1_get_docs_in_random_order_with_limit,
                )
                .await?;
                //todo remove option
                return Ok(Some(vec_of_strings));
            }
        }
        Ok(None)
    }
}