use std::fs::File;
use std::io::prelude::*;

use crate::tests::tests_constants::CONFIG_FILE_EXTENSION;
use crate::tests::tests_constants::USER_CREDENTIALS_CONTENT;
use config_lib::get_project_information::project_constants::PATH_TO_CONFIG_FOR_TEST;
use config_lib::get_project_information::project_constants::USER_CREDENTIALS_FILE_NAME;

#[test]
pub fn ci_check_new_user_credentials_fields() {
    let result_of_opening_file = File::open(format!(
        "{}{}{}",
        PATH_TO_CONFIG_FOR_TEST, USER_CREDENTIALS_FILE_NAME, CONFIG_FILE_EXTENSION
    ));
    match result_of_opening_file {
        Ok(mut file) => {
            let mut contents = String::new();
            let result_of_reading_to_string = file.read_to_string(&mut contents);
            match result_of_reading_to_string {
                Ok(_) => {
                    assert_eq!(contents, USER_CREDENTIALS_CONTENT);
                }
                Err(e) => {
                    panic!(
                        "cannot read_to_string from file {}{}, reason: {}",
                        USER_CREDENTIALS_FILE_NAME, CONFIG_FILE_EXTENSION, e
                    )
                }
            }
        }
        Err(e) => {
            panic!(
                "cannot open {}{}, reason: {}",
                USER_CREDENTIALS_FILE_NAME, CONFIG_FILE_EXTENSION, e
            )
        }
    }
}
