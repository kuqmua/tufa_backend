use std::fs::File;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::tests::tests_constants::PATH_TO_ENV_FILE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_env_file_exists() {
    if let Err(e) = File::open(format!("{}{}", PATH_TO_ENV_FILE, ENV_FILE_NAME)) {
        panic!("file: {}{} error: {}", PATH_TO_ENV_FILE, ENV_FILE_NAME, e);
    }
}
