mod check_net {
    pub mod check_link_status_code;
    pub mod check_net_error_enum;
    pub mod check_net_wrapper;
}
mod constants {
    pub mod env_var_names_constants;
    pub mod project_constants;
    pub mod tests_constants;
}
mod fetch {
    pub mod parse_github_html;
    pub mod rss_async_write_fetch_error_logs_into_files_wrapper;
    pub mod rss_fetch_and_parse_provider_data;
    pub mod rss_fetch_link;
    pub mod rss_filter_fetched_and_parsed_posts;
    pub mod rss_handle_error_status_code;
    pub mod rss_metainfo_fetch_structures;
    pub mod rss_parse_string_into_struct;
    pub mod rss_part;
    pub mod write_provider_json_into_file;
    pub mod info_structures {
        pub mod structs_for_parsing {
            pub mod arxiv_struct_for_parsing;
            pub mod biorxiv_struct_for_parsing;
            pub mod github_struct_for_parsing;
            pub mod habr_struct_for_parsing;
            pub mod medrxiv_struct_for_parsing;
            pub mod reddit_struct_for_parsing;
            pub mod twitter_struct_for_parsing;
        }
        pub mod common_rss_structures;
    }
}
pub mod config_mods {
    pub mod config_error_mods {
        pub mod config_env_var_error_type_enum;
        pub mod config_error;
        pub mod config_error_enum;
        pub mod config_error_inner_type_enum;
        pub mod var_or_bool_parse_error_enum;
        pub mod var_or_int_parse_error_enum;
    }
    pub mod config_values_types_enums {
        pub mod env_var_bool_enum;
        pub mod env_var_i64_enum;
        pub mod env_var_string_enum;
        pub mod env_var_u8_enum;
    }
    pub mod config_structs {
        pub mod config_struct;
        pub mod enable_error_providers_prints_struct;
        pub mod enable_initialize_mongo_with_providers_link_parts_struct;
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
        pub mod github_authorization_struct;
        pub mod mongo_authorization_struct;
        pub mod mongo_params_struct;
        pub mod mongo_url_parts_struct;
        pub mod params_struct;
        pub mod postgres_authorization_struct;
        pub mod postgres_params_struct;
        pub mod postgres_url_parts_struct;
        pub mod print_colors_struct;
        pub mod providers_check_links_struct;
        pub mod providers_links_limits_struct;
        pub mod reddit_authorization_struct;
    }
    pub mod config;
    pub mod env_var_enum;
}
pub mod helpers {
    pub mod create_dir_if_it_doesnt_exist;
    pub mod resource;
    pub mod write_json_into_file;
    pub mod write_string_into_file;
}
mod logs_logic {
    pub mod async_write_fetch_error_logs_into_mongo_wrapper;
    pub mod drop_mongo_provider_logs_collection_if_need;
}
pub mod mongo_integration {
    pub mod mongo_check_availability;
    pub mod mongo_check_collection_is_empty;
    pub mod mongo_check_db_is_empty;
    pub mod mongo_drop_collection;
    pub mod mongo_drop_db;
    pub mod mongo_drop_empty_collection;
    pub mod mongo_drop_empty_db;
    pub mod mongo_get_db_url;
    pub mod mongo_get_documents_as_string_vector;
    pub mod mongo_insert_data;
    pub mod mongo_insert_docs_in_empty_collection;
}
pub mod postgres_integration {
    pub mod models {
        pub mod insertable {
            pub mod new_post;
        }
        pub mod queryable {
            pub mod post;
        }
    }
    pub mod postgres_check_availability;
    pub mod postgres_delete_post;
    pub mod postgres_get_db_url;
    pub mod schema;
}
pub mod prints {
    pub mod print_colorful_message;
    pub mod print_type_enum;
}
mod providers {
    pub mod providers_info {
        pub mod links {
            pub mod generate_arxiv_links;
            pub mod generate_biorxiv_links;
            pub mod generate_github_links;
            pub mod generate_habr_links;
            pub mod generate_medrxiv_links;
            pub mod generate_reddit_links;
            pub mod generate_twitter_links;
        }
        pub mod get_providers_link_parts;
        pub mod get_twitter_provider_name;
        pub mod providers_init_json_schema;
    }
    pub mod provider_kind {
        pub mod get_string_name;
        pub mod get_mongo_log_collection_name;
        pub mod is_enabled;
        pub mod is_prints_enabled;
        pub mod is_mongo_initialization_enabled;
        pub mod is_cleaning_warning_logs_db_collections_in_mongo_enabled;
        pub mod stringify;
        pub mod get_enabled_string_name_vec;
        pub mod get_enabled_providers_vec;
        pub mod get_mongo_initialization_string_name_vec;
        pub mod get_mongo_initialization_provider_kind_vec;
        pub mod into_vec;
        pub mod into_string_name_and_kind_tuple_vec;
    }
    pub mod provider_kind_enum;
}
#[cfg(test)]
mod tests {
    pub mod continuous_integration {
        pub mod ci_check_compromised_github_auth_info_from_env;
        pub mod ci_check_compromised_github_auth_info_with_config_init;
        pub mod ci_check_compromised_reddit_auth_info_from_env;
        pub mod ci_check_compromised_reddit_auth_info_with_config_init;
        pub mod ci_check_docker_compose_changes;
        pub mod ci_check_env_file_exists;
        pub mod ci_check_new_env_vars;
    }
}
mod async_tokio_wrapper;
mod check_new_posts_threads_parts;
mod entry;
mod providers_new_posts_check;
mod write_error_posts_wrapper;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

extern crate dotenv;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
fn main() {
    entry::entry();
}
