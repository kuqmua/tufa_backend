use crate::tests::constants::ENV_FILE_NAME;
use crate::tests::constants::PATH_TO_ENV_FILE;
use std::fs::File;

#[test]
fn ci_check_env_file_exists() {
    if let Err(e) = File::open(format!("{PATH_TO_ENV_FILE}{ENV_FILE_NAME}")) {
        panic!("file: {PATH_TO_ENV_FILE}{ENV_FILE_NAME} error: {e}");
    }
}
