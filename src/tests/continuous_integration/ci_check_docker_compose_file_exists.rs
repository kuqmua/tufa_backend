use std::fs;

use crate::tests::tests_constants::DOCKER_COMPOSE_FILE_NAME;
use crate::tests::tests_constants::PATH_TO_DOCKER_COMPOSE_FILE;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
#[test]
fn ci_check_docker_compose_file_exists() {
    if let Err(e) = fs::read_to_string(&format!(
        "{PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME}"
    )) {
        panic!("file: {PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME} error: {e}");
    }
}
