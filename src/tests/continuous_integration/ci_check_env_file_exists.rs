use std::fs::File;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::constants::tests_constants::_PATH_TO_ENV_FILE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_env_file_exists() {
    let file = File::open(format!(
        "{}{}",
        _PATH_TO_ENV_FILE, ENV_FILE_NAME
    ));
    match file {
        Ok(_) => {}
        Err(e) => panic!("file: {}{} error: {}", _PATH_TO_ENV_FILE, ENV_FILE_NAME, e),
    }
}
