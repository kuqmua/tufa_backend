extern crate toml;

use crate::config_mods::config_error_mods::config_error::ConfigError;

use crate::config_mods::config_struct::ConfigStruct;

impl ConfigStruct {
    pub fn wrap_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError> {
        // if !config_handle.github_authorization.github_name.is_empty() {
        //     let error: Result<ConfigStruct, ConfigError> =
        //         Err(ConfigError::Message("github_name is not valid".to_string()));
        //     drop(error);
        // }
        // if !config_handle.github_authorization.github_token.is_empty() {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "github_token is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        //     .reddit_authorization
        //     .reddit_user_agent
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "reddit_user_agent is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        //     .reddit_authorization
        //     .reddit_client_id
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "reddit_client_id is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        //     .reddit_authorization
        //     .reddit_client_secret
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "reddit_client_secret is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        //     .reddit_authorization
        //     .reddit_username
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "reddit_username is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        //     .reddit_authorization
        //     .reddit_password
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "reddit_password is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        //     .mongo_params.mongo_authorization
        //     .mongo_login
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "mongo_login is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        // .mongo_params
        //     .mongo_authorization
        //     .mongo_password
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "mongo_password is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        // .mongo_params
        //     .mongo_authorization
        //     .mongo_ip
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "mongo_ip is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        // .mongo_params
        //     .mongo_authorization
        //     .mongo_port
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "mongo_port is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !config_handle
        // .mongo_params
        //     .mongo_authorization
        //     .mongo_params
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "mongo_params is not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        // //
        // if config_handle.mongo_params.log_file_extension.is_empty() {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "log_file_extension is not empty".to_string(),
        //     ));
        //     drop(error);
        // }
        // if config_handle
        //     .mongo_params
        //     .path_to_provider_link_parts_folder
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "path_to_provider_link_parts_folder is empty"
        //             .to_string(),
        //     ));
        //     drop(error);
        // }
        // if config_handle
        //     .mongo_params
        //     .providers_db_collection_document_field_name_handle
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "db_collection_document_field_name_handle is empty"
        //             .to_string(),
        //     ));
        //     drop(error);
        // }
        // if config_handle
        //     .mongo_params
        //     .providers_db_collection_handle_second_part
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "db_collection_handle_second_part is empty"
        //             .to_string(),
        //     ));
        //     drop(error);
        // }
        // if config_handle
        //     .mongo_params
        //     .providers_db_name_handle
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "db_name_handle is empty".to_string(),
        //     ));
        //     drop(error);
        // }
        // if config_handle
        //     .params
        //     .unhandled_success_handled_success_are_there_items_initialized_posts_dir
        //     .is_empty()
        // {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //             "unhandled_success_handled_success_are_there_items_initialized_posts_dir is empty".to_string(),
        //         ));
        //     drop(error);
        // }
        // if config_handle.params.warning_logs_directory_name.is_empty() {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "warning_logs_directory_name is empty".to_string(),
        //     ));
        //     drop(error);
        // }
        // if config_handle.params.common_providers_links_limit > 0 {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "common_providers_links_limit <= 0".to_string(),
        //     ));
        //     drop(error);
        // }
        // if !ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&config_handle) {
        //     let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
        //         "providers_links_limits are not valid".to_string(),
        //     ));
        //     drop(error);
        // }
        Ok(config_handle)
    }
}
