pub mod get_project_information {
    pub mod get_config {
        pub mod structures_definitions {
            pub mod config_struct_def;
            pub mod enable_error_providers_prints_def;
            pub mod enable_partial_success_providers_prints_def;
            pub mod enable_providers_cleaning_warning_logs_directory_def;
            pub mod enable_providers_def;
            pub mod enable_providers_links_limit_def;
            pub mod enable_providers_prints_def;
            pub mod enable_providers_time_measurement_def;
            pub mod enable_randomize_order_for_providers_link_parts_for_mongo_def;
            pub mod enable_success_providers_prints_def;
            pub mod enable_warning_high_providers_prints_def;
            pub mod enable_warning_low_providers_prints_def;
            pub mod env_def;
            pub mod links_def;
            pub mod mongo_params_def;
            pub mod params_def;
            pub mod print_colors_def;
            pub mod providers_links_limits_def;
        }
        pub mod structures_implementations {
            pub mod config_struct_impl;
            pub mod env_impl;
        }
        pub mod get_config_information;
    }
    pub mod get_user_credentials {
        pub mod structures_definitions {
            pub mod github_authorization_def;
            pub mod mongo_cloud_authorization_def;
            pub mod mongo_own_authorization_def;
            pub mod reddit_authorization_def;
            pub mod user_credentials_struct_def;
        }
        pub mod structures_implementations {
            pub mod user_credentials_struct_impl;
        }
        pub mod get_user_credentials_information;
    }
    pub mod project_constants;
    pub mod provider_kind_enum;
}

#[macro_use]
extern crate lazy_static;
