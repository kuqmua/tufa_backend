use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::get_config::structures_definitions::config_struct_def::ConfigStruct;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use crate::mongo_get_possible_aggregation_with_randomization_doc_for_provider_wrapper::mongo_get_possible_aggregation_with_randomization_doc_for_provider_wrapper;
use crate::mongo_possibly_get_documents_as_string_vector::mongo_possibly_get_documents_as_string_vector;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main]
pub async fn mongo_get_provider_link_parts_as_bson_string(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_name_handle: String,
    db_collection_document_field_name_handle: &str,
    provider_kind: ProviderKind,
) -> Result<Vec<String>, mongodb::error::Error> {
    //todo maybe option vec string
    let cloned_config = CONFIG.clone(); //todo maybe later remove clone somehow??
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
                                //rewrite as PrintType::Info or something
                                print_colorful_message(
                                    None,
                                    PrintType::Success,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("collection.count_documents {}", documents_number),
                                );
                                let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>;
                                if CONFIG.params.enable_provider_links_limit {
                                    if CONFIG.params.enable_common_providers_links_limit {
                                        if CONFIG.params.enable_randomize_order_for_providers_link_parts_for_mongo {
                                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG.params.common_providers_links_limit }});
                                        }
                                        else {
                                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$limit" :  CONFIG.params.common_providers_links_limit });
                                        }
                                    } else {
                                        option_aggregation_stage_1_get_docs_in_random_order_with_limit = mongo_get_possible_aggregation_with_randomization_doc_for_provider_wrapper(
                                                    cloned_config,
                                                    provider_kind
                                        );
                                    }
                                } else {
                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                }
                                // let aggregation_stage_1_get_docs_in_random_order_with_limit =
                                //     doc! { "$sample" : {"size": 5 }};
                                // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
                                let result = mongo_possibly_get_documents_as_string_vector(
                                    collection,
                                    db_collection_document_field_name_handle,
                                    option_aggregation_stage_1_get_docs_in_random_order_with_limit,
                                )
                                .await;
                                match result {
                                    Ok(vec_of_strings) => vec_of_strings_to_return = vec_of_strings,
                                    Err(e) => {
                                        vec_of_strings_to_return = Vec::new();
                                        print_colorful_message(
                                            None,
                                            PrintType::WarningLow,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!("mongo_possibly_get_documents_as_string_vector error {:#?}", e),
                                        );
                                    }
                                }
                            } else {
                                vec_of_strings_to_return = Vec::new();
                                print_colorful_message(
                                    None,
                                    PrintType::WarningLow,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("documents_number is {}", documents_number),
                                );
                            }
                        }
                        Err(e) => {
                            vec_of_strings_to_return = Vec::new();
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                file!().to_string(),
                                line!().to_string(),
                                format!("collection.count_documents error {:#?}", e),
                            );
                        }
                    }
                }
                None => {
                    vec_of_strings_to_return = Vec::new();
                    print_colorful_message(
                        None,
                        PrintType::WarningLow,
                        file!().to_string(),
                        line!().to_string(),
                        "needed_db_collection is None".to_string(),
                    );
                }
            }
        }
        Err(e) => {
            vec_of_strings_to_return = Vec::new();
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("Client::with_options error {:#?}", e),
            );
        }
    }
    Ok(vec_of_strings_to_return)
}
