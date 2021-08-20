use crate::get_project_information::get_providers_json_local_data::get_providers_json_local_data;
// use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn put_data_in_mongo(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle_second_part: &str,
    db_collection_document_field_name_handle: &str,
    path_to_provider_link_parts_folder: &str,
    vec_of_provider_names: Vec<String>,
    file_extension: &str,
) {
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
        path_to_provider_link_parts_folder,
        vec_of_provider_names,
        db_collection_handle_second_part,
        file_extension,
    );
    let mut result_flag = true;
    if !vec_of_link_parts_hashmap.is_empty() {
        for (key, vec_of_link_parts) in vec_of_link_parts_hashmap {
            let future_inserting_docs = mongo_insert_docs_in_empty_collection(
                mongo_url,
                db_name_handle,
                format!("{}{}", key, db_collection_handle_second_part),
                db_collection_document_field_name_handle,
                vec_of_link_parts,
            )
            .await;
            match future_inserting_docs {
                Ok(_) => { //todo
                }
                Err(e) => {
                    result_flag = false;
                    println!("future_inserting_docs error {:#?}", e);
                }
            }
        }
    } else {
        println!(
            "vec_of_link_parts_hashmap.len() {}",
            vec_of_link_parts_hashmap.len()
        );
        result_flag = false;
    }
    // result_flag
}
