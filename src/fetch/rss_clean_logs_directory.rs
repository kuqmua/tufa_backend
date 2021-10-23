use crate::get_project_information::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::get_project_information::get_config::config::CONFIG;
use std::fs;
use std::path::Path;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_clean_logs_directory(provider_kind: ProviderKind) {
    let path = format!(
        "logs/{}/{:?}",
        &CONFIG.params.warning_logs_directory_name, provider_kind
    );
    if Path::new(&path).is_dir() {
        let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
        match result_of_recursively_removing_warning_logs_directory {
            Ok(_) => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Success,
                    file!().to_string(),
                    line!().to_string(),
                    format!("folder {} has been deleted", &path),
                );
            }
            Err(e) => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("delete folder problem{} {}", &path, e.to_string()),
                );
            }
        }
    }
}
