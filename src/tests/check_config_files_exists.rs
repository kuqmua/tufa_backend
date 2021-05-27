use crate::project_constants::VECTOR_OF_MODES;
use std::fs::File;
#[test]
fn check_config_files_exists() {
    let start: &str = "./config/"; //move this into project constants
    let end: &str = ".toml";
    for mode in VECTOR_OF_MODES {
        let file = File::open(format!("{}{}{}", start, mode, end));
        match file {
            Ok(_) => {}
            Err(e) => panic!("file: {} error: {}", mode, e),
        }
    }
}
