use config_lib::get_project_information::project_constants::LOGS_COMMON_FOLDER_NAME;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::fs;
use std::path::Path;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn create_dir_if_dont_exists(
    underdirectory: &str,
    provider_kind_option: Option<&ProviderKind>,
    warning_logs_directory_name: &str,
) {
    let path_to_log_file: String;
    match provider_kind_option {
        Some(provider_kind) => {
            path_to_log_file = format!(
                "logs/{}/{:?}/{}",
                warning_logs_directory_name, provider_kind, underdirectory
            );
        }
        None => {
            path_to_log_file = format!(
                "logs/{}/{:?}/{}",
                warning_logs_directory_name, LOGS_COMMON_FOLDER_NAME, underdirectory
            );
        }
    }
    if !Path::new(&path_to_log_file).exists() {
        let result_of_creating_directory = fs::create_dir_all(path_to_log_file);
        match result_of_creating_directory {
            Ok(_) => match provider_kind_option {
                Some(provider_kind) => {
                    print_colorful_message(
                        Some(provider_kind),
                        PrintType::Success,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "folder logs/{}/{:?}/{} created!",
                            warning_logs_directory_name, provider_kind, underdirectory
                        ),
                    );
                }
                None => {
                    print_colorful_message(
                        None,
                        PrintType::Success,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "folder logs/{}/{:?}/{} created!",
                            warning_logs_directory_name, LOGS_COMMON_FOLDER_NAME, underdirectory
                        ),
                    );
                }
            },
            Err(e) => match provider_kind_option {
                Some(provider_kind) => {
                    print_colorful_message(
                        Some(provider_kind),
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "folder creation error logs/{}/{:?}/{} {:#?}",
                            warning_logs_directory_name, provider_kind, underdirectory, e
                        ),
                    );
                }
                None => {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "folder creation error logs/{}/{:?}/{} {:#?}",
                            warning_logs_directory_name, LOGS_COMMON_FOLDER_NAME, underdirectory, e
                        ),
                    );
                }
            },
        }
    }
}
