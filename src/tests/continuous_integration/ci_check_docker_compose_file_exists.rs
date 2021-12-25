use std::fs;

use crate::constants::tests_constants::_DOCKER_COMPOSE_FILE_NAME;
use crate::constants::tests_constants::_PATH_TO_DOCKER_COMPOSE_FILE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_docker_compose_file_exists() {
    if let Err(e) = fs::read_to_string(&format!(
        "{}{}",
        _PATH_TO_DOCKER_COMPOSE_FILE, _DOCKER_COMPOSE_FILE_NAME
    )) {
        panic!(
            "file: {}{} error: {}",
            _PATH_TO_DOCKER_COMPOSE_FILE, _DOCKER_COMPOSE_FILE_NAME, e
        );
    }
}
