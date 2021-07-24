use std::fs::File;
use std::io::prelude::*;

use crate::tests::tests_constants::CONFIG_CONTENT;
use crate::tests::tests_constants::CONFIG_FILE_EXTENSION;
use crate::tests::tests_constants::VECTOR_OF_MODES;

use config_lib::get_project_information::project_constants::PATH_TO_CONFIG_FOR_TEST;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_new_config_fields() {
    for mode in VECTOR_OF_MODES {
        let result_of_opening_file = File::open(format!(
            "{}{}{}",
            PATH_TO_CONFIG_FOR_TEST, mode, CONFIG_FILE_EXTENSION
        ));
        match result_of_opening_file {
            Ok(mut file) => {
                let mut contents = String::new();
                let result_of_reading_to_string = file.read_to_string(&mut contents);
                match result_of_reading_to_string {
                    Ok(_) => {
                        assert_eq!(contents, CONFIG_CONTENT);
                    }
                    Err(e) => {
                        panic!(
                            "cannot read_to_string from file {}{}, reason: {}",
                            mode, CONFIG_FILE_EXTENSION, e
                        )
                    }
                }
            }
            Err(e) => {
                panic!(
                    "cannot open {}{}, reason: {}",
                    mode, CONFIG_FILE_EXTENSION, e
                )
            }
        }
    }
}
