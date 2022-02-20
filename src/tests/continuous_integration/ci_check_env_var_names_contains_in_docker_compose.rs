use std::fs;

use strum::IntoEnumIterator;

use crate::tests::tests_constants::DOCKER_COMPOSE_FILE_NAME;
use crate::tests::tests_constants::PATH_TO_DOCKER_COMPOSE_FILE;

use crate::config_mods::config_struct::ConfigStructEnum;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
#[test]
pub fn ci_check_env_var_names_contains_in_docker_compose() {
    let result_of_reading_to_string = fs::read_to_string(&format!(
        "{PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME}"
    ));
    match result_of_reading_to_string {
        Err(e) => {
            panic!("cannot read_to_string from file {PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME}, reason: {e}");
        }
        Ok(file_content) => {
            let mut vec = Vec::with_capacity(ConfigStructEnum::get_length());
            for i in ConfigStructEnum::iter() {
                let env_name = i.to_upper_snake_case();
                if !file_content.contains(&env_name) {
                    vec.push(env_name);
                }
            }
            if !vec.is_empty() {
                panic!("no such env name(s) inside docker-compose: {vec:?}");
            }
        }
    }
}
