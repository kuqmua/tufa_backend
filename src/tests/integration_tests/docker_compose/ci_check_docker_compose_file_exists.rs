use crate::tests::constants::DOCKER_COMPOSE_FILE_NAME;
use crate::tests::constants::PATH_TO_DOCKER_COMPOSE_FILE;
use std::fs;

#[test]
fn ci_check_docker_compose_file_exists() {
    if let Err(e) = fs::read_to_string(&format!(
        "{PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME}"
    )) {
        panic!("file: {PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME} error: {e}");
    }
}
