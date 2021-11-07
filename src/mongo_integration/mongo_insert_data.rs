use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
use crate::providers::providers_info::get_providers_json_local_data::get_providers_json_local_data;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;

pub enum PutDataInMongoResult {
    Success,
    PartialSuccess, //todo: add values in here
    Failure,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_insert_data(db_name_handle: &str) -> PutDataInMongoResult {
    let vec_of_link_parts_hashmap = ProviderKind::get_providers_json_local_data();
    if vec_of_link_parts_hashmap.is_empty() {
        println!(
            "vec_of_link_parts_hashmap.len() {}",
            vec_of_link_parts_hashmap.len()
        );
        return PutDataInMongoResult::Failure;
    }
    let mut vec_of_futures = Vec::with_capacity(vec_of_link_parts_hashmap.len());
    //todo: add case add in non empty collection
    for (provider_kind, vec_of_link_parts) in vec_of_link_parts_hashmap {
        let future_inserting_docs = mongo_insert_docs_in_empty_collection(
            db_name_handle,
            format!(
                "{}{}",
                ProviderKind::get_string_name(provider_kind),
                CONFIG
                    .mongo_params
                    .providers_db_collection_handle_second_part
            ),
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
