use std::fs::File;
use std::io::prelude::*;

use config_lib::get_project_information::project_constants::ENV_FILE_NAME;

use crate::tests::tests_constants::ENV_FILE_CONTENT;
use crate::tests::tests_constants::PATH_TO_ENV_FILE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_new_env_vars() {
    let result_of_opening_file = File::open(format!(
        "{}{}",
        PATH_TO_ENV_FILE, ENV_FILE_NAME
    ));
    match result_of_opening_file {
        Ok(mut file) => {
            let mut contents = String::new();
            let result_of_reading_to_string = file.read_to_string(&mut contents);
            match result_of_reading_to_string {
                Ok(_) => {
                    assert_eq!(contents, ENV_FILE_CONTENT);
                }
                Err(e) => {
                    panic!(
                        "cannot read_to_string from file {}{}, reason: {}",
                        PATH_TO_ENV_FILE, ENV_FILE_NAME, e
                    )
                }
            }
        }
        Err(e) => {
            panic!(
                "cannot open {}{}, reason: {}",
                PATH_TO_ENV_FILE, ENV_FILE_NAME, e
            )
        }
    }
}
