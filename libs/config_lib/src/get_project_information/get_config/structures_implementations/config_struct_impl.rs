use crate::get_project_information::get_config::structures_definitions::config_struct_def::ConfigStruct;
use crate::get_project_information::project_constants::PROJECT_MODE;
use crate::get_project_information::provider_kind_enum::ProviderKind;
use config::{Config, ConfigError, File};

impl ConfigStruct {
    pub fn new(mode_handler: Option<&str>, path_to_config: &str) -> Result<Self, ConfigError> {
        match mode_handler {
            Some(mode) => {
                //for tests - maybe remove and copy code for testing later but its more comfortable for now
                let mut config = Config::new();
                match config.set("env", mode) {
                    Ok(_) => {
                        match config.merge(File::with_name(&format!("{}{}", path_to_config, mode)))
                        {
                            Ok(_) => {
                                let config_result: Result<Self, ConfigError> = config.try_into();
                                match config_result {
                                    Ok(config_handle) => {
                                        ConfigStruct::wrap_custom_config_checks(config_handle)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            None => {
                // RUN_ENV=Testing cargo run
                let env = std::env::var("RUN_ENV").unwrap_or_else(|_| PROJECT_MODE.into());
                let mut config = Config::new();
                match config.set("env", env.clone()) {
                    Ok(_) => {
                        match config.merge(File::with_name(&format!("{}{}", path_to_config, env))) {
                            Ok(_) => {
                                let config_result: Result<Self, ConfigError> = config.try_into();
                                match config_result {
                                    Ok(config_handle) => {
                                        ConfigStruct::wrap_custom_config_checks(config_handle)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn wrap_custom_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError> {
        let is_common_providers_links_limit_valid =
            ConfigStruct::check_valid_i64_common_providers_links_limit_for_mongo(&config_handle);
        let are_providers_links_limits_valid =
            ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&config_handle);
        if config_handle.mongo_params.file_extension.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.mongo_params.file_extension is not empty".to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .path_to_provider_link_parts_folder
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle
            .mongo_params
            .path_to_provider_link_parts_folder is empty"
                    .to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .db_collection_document_field_name_handle
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle
            .mongo_params
            .db_collection_document_field_name_handle is empty"
                    .to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .db_collection_handle_second_part
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle
            .mongo_params
            .db_collection_handle_second_part is empty"
                    .to_string(),
            ));
            drop(error);
        }
        if config_handle.mongo_params.db_name_handle.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.mongo_params.db_name_handle is empty".to_string(),
            ));
            drop(error);
        }
        if config_handle
            .params
            .unhandled_success_handled_success_are_there_items_initialized_posts_dir
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                    "config_handle.params.unhandled_success_handled_success_are_there_items_initialized_posts_dir is empty".to_string(),
                ));
            drop(error);
        }
        if config_handle.params.warning_logs_directory_name.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.params.warning_logs_directory_name is empty".to_string(),
            ));
            drop(error);
        }
        if !is_common_providers_links_limit_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "common_providers_links_limit is not valid".to_string(),
            ));
            drop(error);
        }
        if !are_providers_links_limits_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "providers_links_limits are not valid".to_string(),
            ));
            drop(error);
        }
        Ok(config_handle)
    }
    fn check_valid_i64_common_providers_links_limit_for_mongo(
        config_handle: &ConfigStruct,
    ) -> bool {
        config_handle.params.common_providers_links_limit > 0
    }
    fn check_valid_i64_providers_links_limits_for_mongo(config_handle: &ConfigStruct) -> bool {
        let mut checker = true;
        if config_handle.providers_links_limits.links_limit_for_arxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_biorxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_github <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_habr <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_medrxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_reddit <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_twitter <= 0 {
            checker = false;
        }
        checker
    }
    pub fn get_links_limit_wrapper_for_provider(self, provider_kind: &ProviderKind) -> i64 {
        match provider_kind {
            ProviderKind::Arxiv => self.providers_links_limits.links_limit_for_arxiv,
            ProviderKind::Biorxiv => self.providers_links_limits.links_limit_for_biorxiv,
            ProviderKind::Github => self.providers_links_limits.links_limit_for_github,
            ProviderKind::Habr => self.providers_links_limits.links_limit_for_habr,
            ProviderKind::Medrxiv => self.providers_links_limits.links_limit_for_medrxiv,
            ProviderKind::Reddit => self.providers_links_limits.links_limit_for_reddit,
            ProviderKind::Twitter => self.providers_links_limits.links_limit_for_twitter,
        }
    }
}
