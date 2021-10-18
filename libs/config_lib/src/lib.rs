pub mod get_project_information {
    pub mod get_config {
        pub mod config_struct;
        pub mod config_error;
        pub mod enable_error_providers_prints_struct;
        pub mod enable_initialize_mongo_with_providers_link_parts_struct;
        pub mod mongo_url_parts_struct;
        pub mod enable_partial_success_providers_prints_struct;
        pub mod enable_providers_cleaning_warning_logs_db_collections_in_mongo_struct;
        pub mod enable_providers_cleaning_warning_logs_db_in_mongo_struct;
        pub mod enable_providers_cleaning_warning_logs_directory_struct;
        pub mod enable_providers_info_struct;
        pub mod enable_providers_links_limit_struct;
        pub mod enable_providers_prints_struct;
        pub mod enable_providers_struct;
        pub mod enable_providers_time_measurement_struct;
        pub mod enable_randomize_order_for_providers_link_parts_for_mongo_struct;
        pub mod enable_success_providers_prints_struct;
        pub mod enable_warning_high_providers_prints_struct;
        pub mod enable_warning_low_providers_prints_struct;
        pub mod get_lazy_config_information;
        pub mod github_authorization_struct;
        pub mod mongo_authorization_struct;
        pub mod mongo_params_struct;
        pub mod params_struct;
        pub mod postgres_authorization_struct;
        pub mod postgres_url_parts_struct;
        pub mod postgres_params_struct;
        pub mod print_colors_struct;
        pub mod providers_check_links_struct;
        pub mod providers_links_limits_struct;
        pub mod reddit_authorization_struct;
    }
    pub mod env_var_enum;
    pub mod env_var_u8_enum;
    pub mod env_var_bool_enum;
    pub mod get_mongo_url;
    pub mod project_constants;
    pub mod env_var_string_names_constants;
    pub mod env_var_bool_names_constants;
    pub mod env_var_i64_names_constants;
    pub mod env_var_u8_names_constants;
    pub mod env_var_names_constants;
    pub mod provider_kind_enum;
}

#[macro_use]
extern crate lazy_static;

extern crate dotenv;