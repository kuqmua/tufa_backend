use std::fs;

use strum::IntoEnumIterator;

use crate::constants::tests_constants::_DOCKER_COMPOSE_FILE_NAME;
use crate::constants::tests_constants::_PATH_TO_DOCKER_COMPOSE_FILE;

use crate::config_mods::env_var_enum::EnvVar;
use crate::traits::enum_extention::EnumExtenstion;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_env_var_names_contains_in_docker_compose() {
    let result_of_reading_to_string = fs::read_to_string(&format!(
        "{}{}",
        _PATH_TO_DOCKER_COMPOSE_FILE, _DOCKER_COMPOSE_FILE_NAME
    ));
    match result_of_reading_to_string {
        Err(e) => {
            panic!(
                "cannot read_to_string from file {}{}, reason: {}",
                _PATH_TO_DOCKER_COMPOSE_FILE, _DOCKER_COMPOSE_FILE_NAME, e
            )
        }
        Ok(file_content) => {
            let mut vec = Vec::with_capacity(EnvVar::get_length());
            for i in EnvVar::iter() {
                let env_name = i.to_upper_snake_case();
                if !file_content.contains(&env_name) {
                    vec.push(env_name);
                }
            }
            if !vec.is_empty() {
                panic!("no such env name(s) inside docker-compose: {:?}", vec);
            }
        }
    }
}
