use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::fs;
use std::path::Path;
// use crate::constants::project_constants::LOGS_COMMON_FOLDER_NAME;
// path_to_log_file = format!(
//     "logs/{}/{:?}/{}",
//     warning_logs_directory_name, LOGS_COMMON_FOLDER_NAME, underdirectory
// )
#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn create_provider_logs_dir_if_dont_exists(
    path_to_provider_log_file: String,
    provider_kind: ProviderKind,
) -> Result<(), std::io::Error> {
    if Path::new(&path_to_provider_log_file).exists() {
        return Ok(());
    }
    let result_of_creating_directory = fs::create_dir_all(&path_to_provider_log_file);
    if let Err(e) = result_of_creating_directory {
        print_colorful_message(
            Some(&provider_kind),
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!(
                "folder creation path is not valid: {}, error: {:#?}",
                path_to_provider_log_file, e
            ),
        );
        return Err(e);
    }
    Ok(())
}
