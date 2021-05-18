use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::get_project_information::get_config::get_config_information::CONFIG;
use crate::overriding::prints::print_error_red;
use std::fs;
use std::path::Path;

pub fn rss_clean_logs_directory(
    provider_kind: ProviderKind,
    enable_prints: bool,
    enable_error_prints: bool,
) {
    let path = format!(
        "logs/{}/{:?}",
        &CONFIG.params.warning_logs_directory_name, provider_kind
    );
    if Path::new(&path).is_dir() {
        let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
        match result_of_recursively_removing_warning_logs_directory {
            Ok(_) => {
                if enable_prints {
                    println!("folder {} has been deleted", &path);
                }
            }
            Err(e) => {
                if enable_error_prints {
                    let error_message = format!("delete folder problem{} {}", &path, e.to_string());
                    print_error_red(file!().to_string(), line!().to_string(), error_message)
                }
            }
        }
    }
}
