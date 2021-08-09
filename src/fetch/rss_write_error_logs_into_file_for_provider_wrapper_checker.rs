use serde_json::Value;

use crate::fetch::rss_write_error_logs_into_file_for_provider::rss_write_error_logs_into_file_for_provider;
use crate::helpers::create_dir_if_dont_exists::create_dir_if_dont_exists;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

//under development
// use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
// use config_lib::get_project_information::get_user_credentials::get_lazy_user_credentials_information::USER_CREDENTIALS;
// use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
// use mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
//under development

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::fs::File;
use std::io::ErrorKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_write_error_logs_into_file_for_provider_wrapper_checker(
    json_object: Value,
    provider_kind: &ProviderKind,
    dir: &str,
    warning_logs_directory_name: &str,
    link: &str,
) {
    create_dir_if_dont_exists(dir, Some(&provider_kind), &warning_logs_directory_name);
    let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
    let file_name = format!(
        "logs/{}/{:?}/{}/{:?}-{}.json",
        &warning_logs_directory_name, provider_kind, dir, provider_kind, replaced_link
    ); //add save function what convert string into save path
    let result_of_opening_file = File::open(&file_name);
    match result_of_opening_file {
        Ok(_) => {
            print_colorful_message(
                Some(provider_kind),
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "there is file with the same name: {}, file was not written",
                    &file_name
                ),
            );
        }
        Err(ref err) => {
            if err.kind() == ErrorKind::NotFound {
                //todo write into mongo collection and create flag where to write logs
                rss_write_error_logs_into_file_for_provider(file_name, json_object);
            } else {
                print_colorful_message(
                    Some(provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "unexpected error while opening file, description: {:#?}",
                        &err.kind()
                    ),
                );
                //todo write into mongo collection and create flag where to write logs
                // let mongo_cloud_first_handle_url_part =
                //     &CONFIG.mongo_params.mongo_cloud_first_handle_url_part;
                // let mongo_cloud_login =
                //     &USER_CREDENTIALS.mongo_cloud_authorization.mongo_cloud_login;
                // let mongo_cloud_second_handle_url_part =
                //     &CONFIG.mongo_params.mongo_cloud_second_handle_url_part;
                // let mongo_cloud_password = &USER_CREDENTIALS
                //     .mongo_cloud_authorization
                //     .mongo_cloud_password;
                // let mongo_cloud_third_handle_url_part =
                //     &CONFIG.mongo_params.mongo_cloud_third_handle_url_part;
                // let mongo_cloud_cluster_name = &USER_CREDENTIALS
                //     .mongo_cloud_authorization
                //     .mongo_cloud_cluster_name;
                // let mongo_cloud_fourth_handle_url_part =
                //     &CONFIG.mongo_params.mongo_cloud_fourth_handle_url_part;
                // let mongo_cloud_cluster_params = &USER_CREDENTIALS
                //     .mongo_cloud_authorization
                //     .mongo_cloud_cluster_params;
                // let mongo_url = format!(
                //     "{}{}{}{}{}{}{}{}",
                //     mongo_cloud_first_handle_url_part,
                //     mongo_cloud_login,
                //     mongo_cloud_second_handle_url_part,
                //     mongo_cloud_password,
                //     mongo_cloud_third_handle_url_part,
                //     mongo_cloud_cluster_name,
                //     mongo_cloud_fourth_handle_url_part,
                //     mongo_cloud_cluster_params
                // );
                // let db_name_handle = "logs";
                // let db_collection_name = "arxiv";
                // let check_if_collection_empty = false;
                // let db_collection_document_field_name_handle = "data";
                // let vec_of_link_parts = vec_of_values: Vec<String>, but it must be doc! type
                ////////////////////////////////
                //here only one insert. Need many
                ////////////////////////////////
                //////////
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
                ////////////////////////////////////////////////////////////////////////////////////////////////
                //     let future_inserting_docs = mongo_insert_docs_in_empty_collection(
                //     mongo_url,
                //     db_name_handle,
                //     &format!("{}{}", key, db_collection_handle_second_part),
                //     db_collection_document_field_name_handle,
                //     vec_of_link_parts,
                // );
                // match future_inserting_docs {
                //     Ok(_) => {}
                //     Err(e) => {
                //         result_flag = false;
                //         println!("future_inserting_docs error {:#?}", e);
                //     }
                // }
                rss_write_error_logs_into_file_for_provider(file_name, json_object);
            }
        }
    }
}
