use std::fs;

use config_lib::get_project_information::project_constants::ENV_FILE_NAME;

use crate::tests::tests_constants::ENV_FILE_CONTENT;
use crate::tests::tests_constants::PATH_TO_ENV_FILE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_new_env_vars() {
    let result_of_reading_to_string = fs::read_to_string(&format!(
        "{}{}",
        PATH_TO_ENV_FILE, ENV_FILE_NAME
    ));
    match result_of_reading_to_string {
        Ok(file_content) => {
            assert_eq!(file_content, ENV_FILE_CONTENT);
        }
        Err(e) => {
            panic!(
                "cannot read_to_string from file {}{}, reason: {}",
                PATH_TO_ENV_FILE, ENV_FILE_NAME, e
            )
        }
    }
}
