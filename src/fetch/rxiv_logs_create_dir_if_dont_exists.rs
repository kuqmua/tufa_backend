use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::overriding::prints::print_error_red;
use std::fs;
use std::path::Path;

pub fn rxiv_logs_create_dir_if_dont_exists(
    underdirectory: &str,
    rxiv_kind: &RxivKind,
    enable_prints: bool,
    enable_error_prints: bool,
) {
    let path_to_log_file = format!(
        "logs/warning_logs/{}/{:?}",
        underdirectory.to_string(),
        rxiv_kind
    );
    if !Path::new(&path_to_log_file).exists() {
        let result_of_creating_directory = fs::create_dir_all(path_to_log_file);
        match result_of_creating_directory {
            Ok(_) => {
                if enable_prints {
                    println!(
                        "папка logs/warning_logs/{:?}/{:?} создана!",
                        underdirectory, rxiv_kind
                    )
                }
            }
            Err(e) => {
                if enable_error_prints {
                    print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                }
            }
        }
    }
}
