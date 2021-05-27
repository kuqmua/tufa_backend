use crate::project_constants::CONFIG_FILE_EXTENSION;
use crate::project_constants::PATH_TO_CONFIG;
use crate::project_constants::VECTOR_OF_MODES;
use std::fs::File;
#[test]
fn check_config_files_exists() {
    for mode in VECTOR_OF_MODES {
        let file = File::open(format!(
            "{}{}{}",
            PATH_TO_CONFIG, mode, CONFIG_FILE_EXTENSION
        ));
        match file {
            Ok(_) => {}
            Err(e) => panic!("file: {} error: {}", mode, e),
        }
    }
}
