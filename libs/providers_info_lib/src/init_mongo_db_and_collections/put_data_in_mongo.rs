use crate::init_mongo_db_and_collections::get_providers_json_local_data::get_providers_json_local_data;
// use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

pub fn put_data_in_mongo() {
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let mongo_url = "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority";
    let db_name_handle = "provider_links";
    let db_collection_handle_second_part = "_link_parts";
    let db_collection_document_field_name_handle = "link_part";
    ////
    let path = "./providers_link_parts/";
    let vec_of_provider_names = vec![
        "arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter",
    ];
    let file_extension = ".json";
    ////
    // let future_possible_drop_collection = mongo_drop_collection_wrapper(
    //     mongo_url,
    //     db_name_handle,
    //     db_collection_document_field_name_handle,
    //     false,
    // );
    // match future_possible_drop_collection {
    //     Ok(result_flag) => {
    //         if result_flag {
    //             println!("drop done!");
    //         } else {
    //             println!("drop fail with flag");
    //         }
    //     }
    //     Err(e) => {
    //         println!("drop fail with error {:#?}", e);
    //     }
    // }
    ////////////////////
    let vec_of_link_parts_hashmap = get_providers_json_local_data(
        path,
        vec_of_provider_names,
        db_collection_handle_second_part,
        file_extension,
    );

    for (key, vec_of_link_parts) in vec_of_link_parts_hashmap {
        let future_inserting_docs = mongo_insert_docs_in_empty_collection(
            mongo_url,
            db_name_handle,
            &format!("{}{}", key, db_collection_handle_second_part),
            db_collection_document_field_name_handle,
            vec_of_link_parts,
        );
        match future_inserting_docs {
            Ok(_) => {
                println!("nice! ");
            }
            Err(e) => {
                println!("future_inserting_docs error {:#?}", e);
            }
        }
    }
}
