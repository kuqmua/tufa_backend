use std::collections::HashMap;

use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::lazy_static_config::CONFIG;

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
    for (pk, vec_of_link_parts) in vec_of_link_parts_hashmap {
        let future_inserting_docs = mongo_insert_docs_in_empty_collection(
            db_name_handle,
            format!(
                "{}{}",
                pk, CONFIG.mongo_providers_logs_db_collection_handle_second_part
            ),
            vec_of_link_parts,
        )
        .await;
        vec_of_futures.push(future_inserting_docs);
    }
    //todo add iteration function which returns Success Partial Success Failure
    let mut checker_if_all_true = true;
    for result in &vec_of_futures {
        if result.is_err() {
            checker_if_all_true = false;
            break;
        }
    }
    if checker_if_all_true {
        return PutDataInMongoResult::Success;
    }
    let mut checker_if_all_false = true;
    for result in &vec_of_futures {
        if result.is_ok() {
            checker_if_all_false = false;
            break;
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
