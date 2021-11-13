use std::collections::HashMap;

use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

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
pub async fn mongo_insert_data(
    db_name_handle: &str,
    vec_of_link_parts_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> PutDataInMongoResult {
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
    for i in 0..vec_of_futures.len() {
        match vec_of_futures.get(i) {
            Some(future_result) => match future_result {
                Ok(boolean_result) => {
                    if !boolean_result {
                        checker_if_all_true = *boolean_result;
                    }
                }
                Err(_) => {
                    checker_if_all_true = false;
                }
            },
            None => {
                //todo: handle this case or rewrite this code
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    "vec_of_futures.get(i) is None. WHY?????".to_string(),
                );
                checker_if_all_true = false;
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
                if *boolean_result {
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
