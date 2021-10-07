use crate::tests::tests_constants::ENV_FILE_NAME;
use crate::tests::tests_constants::PATH_TO_ENV_FILE;
use std::fs::File;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_env_file_exists() {
    let file = File::open(format!(
        "{}{}",
        PATH_TO_ENV_FILE, ENV_FILE_NAME
    ));
    match file {
        Ok(_) => {}
        Err(e) => panic!("file: {}{} error: {}", PATH_TO_ENV_FILE, ENV_FILE_NAME, e),
    }
}
