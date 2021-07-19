use serde_json::Value;

use crate::fetch::rss_logs_create_dir_if_dont_exists::rss_logs_create_dir_if_dont_exists;
use crate::fetch::rss_write_error_logs_into_file_for_provider::rss_write_error_logs_into_file_for_provider;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message;
use prints_lib::PrintType;

use std::fs::File;
use std::io::ErrorKind;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_write_error_logs_into_file_for_provider_wrapper_checker(
    json_object: Value,
    provider_kind: &ProviderKind,
    dir: &str,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    warning_logs_directory_name: &str,
    link: &str,
) {
    rss_logs_create_dir_if_dont_exists(
        dir,
        &provider_kind,
        enable_prints,
        enable_error_prints,
        &warning_logs_directory_name,
    );
    let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
    let file_name = format!(
        "logs/{}/{:?}/{}/{:?}-{}.json",
        &warning_logs_directory_name, provider_kind, dir, provider_kind, replaced_link
    ); //add save function what convert string into save path
    let result_of_opening_file = File::open(&file_name);
    match result_of_opening_file {
        Ok(_) => {
            if enable_warning_prints {
                let warning_message = format!("there is file with the same name: {}", &file_name);
                print_colorful_message(
                    Some(provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    warning_message,
                );
            }
        }
        Err(ref err) => {
            if err.kind() == ErrorKind::NotFound {
                rss_write_error_logs_into_file_for_provider(
                    enable_prints,
                    enable_error_prints,
                    file_name,
                    json_object,
                );
            } else if enable_warning_prints {
                let warning_message = format!(
                    "unexpected error while opening file, description: {:#?}",
                    &err.kind()
                );
                print_colorful_message(
                    Some(provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    warning_message,
                );
                rss_write_error_logs_into_file_for_provider(
                    enable_prints,
                    enable_error_prints,
                    file_name,
                    json_object,
                );
            }
        }
    }
}
