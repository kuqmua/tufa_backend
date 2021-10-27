use crate::providers::providers_info::get_providers_json_local_data::get_providers_json_local_data;
// use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

pub enum PutDataInMongoResult {
    Success,
    PartialSuccess, //todo: add values in here
    Failure,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_insert_data(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_document_field_name_handle: &str,
    path_to_provider_link_parts_folder: &str
) -> PutDataInMongoResult {
    // for key in vec_of_provider_names.clone() {
    //     let future_possible_drop_collection = mongo_drop_collection_wrapper(
    //         mongo_url,
    //         db_name_handle,
    //         &format!("{}{}", key, db_collection_handle_second_part),
    //         false,
    //     );
    //     match future_possible_drop_collection {
    //         Ok(result_flag) => {
    //             if result_flag {
    //                 println!("drop done!");
    //             } else {
    //                 println!("drop fail with flag");
    //             }
    //         }
    //         Err(e) => {
    //             println!("drop fail with error {:#?}", e);
    //         }
    //     }
    // }
    let vec_of_link_parts_hashmap = get_providers_json_local_data(
        path_to_provider_link_parts_folder
    );
    if vec_of_link_parts_hashmap.is_empty() {
        println!(
            "vec_of_link_parts_hashmap.len() {}",
            vec_of_link_parts_hashmap.len()
        );
        return PutDataInMongoResult::Failure;
    }
    let mut vec_of_futures = Vec::with_capacity(vec_of_link_parts_hashmap.len());
    //todo: add case add in non empty collection
    for (key, vec_of_link_parts) in vec_of_link_parts_hashmap {
        let future_inserting_docs = mongo_insert_docs_in_empty_collection(
            mongo_url,
            db_name_handle,
            format!("{}{}", key, CONFIG
            .mongo_params
            .providers_db_collection_handle_second_part),
            db_collection_document_field_name_handle,
            vec_of_link_parts,
        )
        .await;
        vec_of_futures.push(future_inserting_docs);
    }
    //todo add iteration function which returns Success Partial Success Failure
    let mut checker_if_all_true = true;
    for future_result in &vec_of_futures {
        match future_result {
            Ok(boolean_result) => {
                if *boolean_result == false {
                    checker_if_all_true = *boolean_result;
                }
                break;
            }
            Err(_) => {
                checker_if_all_true = false;
                break;
            }
        }
    }
    if checker_if_all_true {
        return PutDataInMongoResult::Success;
    }
    let mut checker_if_all_false = true;
    for future_result in &vec_of_futures {
        match future_result {
            Ok(boolean_result) => {
                if *boolean_result == true {
                    checker_if_all_false = false
                }
            }
            Err(_) => {}
        }
    }
    if checker_if_all_false {
        return PutDataInMongoResult::Failure;
    }
    //todo write more info about this case
    print_colorful_message(
        None,
        PrintType::WarningLow,
        file!().to_string(),
        line!().to_string(),
        "partial_success coz results_vec not all true".to_string(),
    );
    PutDataInMongoResult::PartialSuccess
}
