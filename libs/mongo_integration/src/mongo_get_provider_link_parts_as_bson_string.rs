use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, Document},
    options::{ClientOptions, FindOptions},
    Client,
};

use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main]
pub async fn mongo_get_provider_link_parts_as_bson_string(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_name_handle: String,
    db_collection_document_field_name_handle: &str,
    provider_kind: ProviderKind,
) -> Result<Vec<String>, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client_result = Client::with_options(client_options);
    let vec_of_strings_to_return: Vec<String>;
    match client_result {
        Ok(client) => {
            //declare db name. there is no create db method in mongo
            let db = client.database(db_name_handle);
            let mut needed_db_collection: Option<String> = None;
            for collection_name in db.list_collection_names(None).await? {
                if collection_name == *db_collection_name_handle {
                    needed_db_collection = Some(collection_name);
                }
            }
            match needed_db_collection {
                Some(collection_name) => {
                    let collection = db.collection(&collection_name);
                    let documents_number_result = collection.count_documents(None, None).await;
                    match documents_number_result {
                        Ok(documents_number) => {
                            if documents_number > 0 {
                                println!("collection.count_documents {}", documents_number);
                                let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>;
                                if CONFIG.params.enable_provider_links_limit {
                                    if CONFIG.params.enable_common_providers_links_limit {
                                        option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG.params.common_providers_links_limit }});
                                    } else {
                                        match provider_kind {
                                            ProviderKind::Arxiv => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_arxiv
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_arxiv }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                            ProviderKind::Biorxiv => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_biorxiv
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_biorxiv }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                            ProviderKind::Github => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_github
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_github }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                            ProviderKind::Habr => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_habr
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_habr }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                            ProviderKind::Medrxiv => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_medrxiv
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_medrxiv }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                            ProviderKind::Reddit => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_reddit
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_reddit }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                            ProviderKind::Twitter => {
                                                if CONFIG
                                                    .enable_providers_links_limits
                                                    .enable_links_limit_for_twitter
                                                {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG
                                                                    .providers_links_limits
                                                                    .links_limit_for_twitter }});
                                                } else {
                                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                }
                                match option_aggregation_stage_1_get_docs_in_random_order_with_limit
                                {
                                    Some(
                                        aggregation_stage_1_get_docs_in_random_order_with_limit,
                                    ) => {
                                        // let aggregation_stage_1_get_docs_in_random_order_with_limit =
                                        //     doc! { "$sample" : {"size": 5 }};
                                        // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
                                        let pipeline = vec![
                                            aggregation_stage_1_get_docs_in_random_order_with_limit,
                                        ];
                                        let cursor_result =
                                            collection.aggregate(pipeline, None).await;
                                        match cursor_result {
                                            Ok(mut cursor) => {
                                                let mut vec_of_strings: Vec<String> = Vec::new();
                                                while let Some(document) = cursor.try_next().await?
                                                {
                                                    let bson_option = document.get(
                                                        db_collection_document_field_name_handle,
                                                    );
                                                    match bson_option {
                                                        Some(bson_handle) => match bson_handle {
                                                            bson::Bson::String(
                                                                stringified_bson,
                                                            ) => vec_of_strings
                                                                .push(stringified_bson.to_string()),
                                                            _ => {
                                                                println!("(todo change this print) different mongo type")
                                                            }
                                                        },
                                                        None => {
                                                            println!(
                                                        "no db_collection_document_field_name_handle: {}",
                                                        db_collection_document_field_name_handle
                                                    );
                                                        }
                                                    }
                                                }
                                                if !vec_of_strings.is_empty() {
                                                    vec_of_strings_to_return = vec_of_strings
                                                } else {
                                                    vec_of_strings_to_return = Vec::new()
                                                }
                                            }
                                            Err(e) => {
                                                vec_of_strings_to_return = Vec::new();
                                                println!(
                                            "(todo change this print)  collection.find, {:#?}",
                                            e
                                        )
                                            }
                                        }
                                    }
                                    None => {
                                        let cursor_result = collection.aggregate(None, None).await;
                                        match cursor_result {
                                            Ok(mut cursor) => {
                                                let mut vec_of_strings: Vec<String> = Vec::new();
                                                while let Some(document) = cursor.try_next().await?
                                                {
                                                    let bson_option = document.get(
                                                        db_collection_document_field_name_handle,
                                                    );
                                                    match bson_option {
                                                        Some(bson_handle) => match bson_handle {
                                                            bson::Bson::String(
                                                                stringified_bson,
                                                            ) => vec_of_strings
                                                                .push(stringified_bson.to_string()),
                                                            _ => {
                                                                println!("(todo change this print) different mongo type")
                                                            }
                                                        },
                                                        None => {
                                                            println!(
                                                        "no db_collection_document_field_name_handle: {}",
                                                        db_collection_document_field_name_handle
                                                    );
                                                        }
                                                    }
                                                }
                                                if !vec_of_strings.is_empty() {
                                                    vec_of_strings_to_return = vec_of_strings
                                                } else {
                                                    vec_of_strings_to_return = Vec::new()
                                                }
                                            }
                                            Err(e) => {
                                                vec_of_strings_to_return = Vec::new();
                                                println!(
                                            "(todo change this print)  collection.find, {:#?}",
                                            e
                                        )
                                            }
                                        }
                                    }
                                }
                            } else {
                                vec_of_strings_to_return = Vec::new();
                                println!("documents_number is {}", documents_number)
                            }
                        }
                        Err(e) => {
                            vec_of_strings_to_return = Vec::new();
                            println!(
                                "(todo change this print) collection.count_documents, {:#?}",
                                e
                            )
                        }
                    }
                }
                None => {
                    vec_of_strings_to_return = Vec::new();
                    println!("(todo change this print) no such collection2");
                }
            }
        }
        Err(e) => {
            vec_of_strings_to_return = Vec::new();
            println!("(todo change this print) no client , {:#?}", e);
        }
    }
    println!(
        "vec_of_strings_to_return.len() {}",
        vec_of_strings_to_return.len()
    );
    Ok(vec_of_strings_to_return)
}
