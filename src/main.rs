mod authorization {
    pub mod reddit {
        pub mod reddit_authorization;
    }
}
mod check_net {
    pub mod check_link;
    pub mod check_link_metainfo_structures;
    pub mod fetch_link;
}
mod constants {
    pub mod env_var_names_constants;
    pub mod project_constants;
    pub mod tests_constants;
}
mod fetch {
    pub mod async_write_json_into_file;
    pub mod parse_github_html;
    pub mod provider_log_into_json;
    pub mod rss_async_write_fetch_error_logs_into_files_wrapper;
    pub mod rss_check_available_providers;
    pub mod rss_check_handled_fetch_status_info;
    pub mod rss_check_provider_status;
    pub mod rss_clean_logs_directory;
    pub mod rss_clean_logs_directory_wrapper;
    pub mod rss_divide_to_equal_for_each_provider;
    pub mod rss_fetch_and_parse_provider_data;
    pub mod rss_fetch_link;
    pub mod rss_filter_fetched_and_parsed_posts;
    pub mod rss_handle_error_status_code;
    pub mod rss_handle_unfiltered_posts;
    pub mod rss_metainfo_fetch_structures;
    pub mod rss_parse_string_into_struct;
    pub mod rss_part;
    pub mod rss_write_error_logs_into_file_for_provider_wrapper_checker;
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
    pub mod create_dir_if_dont_exists;
    pub mod json_to_string;
    pub mod resource;
    pub mod write_json_into_file;
    pub mod write_string_into_file;
}
mod logs_logic {
    pub mod async_write_fetch_error_logs_into_mongo_wrapper;
    pub mod drop_mongo_logs_collection_wrapper_for_providers;
    pub mod drop_mongo_provider_logs_collection_if_need;
    pub mod insert_docs_in_empty_mongo_collection_wrapper_under_old_tokio_version;
}
pub mod mongo_integration {
    pub mod mongo_check_collection_is_empty;
    pub mod mongo_drop_collection;
    pub mod mongo_drop_db_checked_on_empty;
    pub mod mongo_drop_db;
    pub mod mongo_drop_empty_collection;
    pub mod mongo_get_db_url;
    pub mod mongo_get_possible_aggregation_with_randomization_doc_for_provider;
    pub mod mongo_get_possible_aggregation_with_randomization_doc_for_provider_wrapper;
    pub mod mongo_insert_data;
    pub mod mongo_insert_docs_in_empty_collection;
    pub mod mongo_possibly_get_documents_as_string_vector;
}
pub mod postgres_integration {
    pub mod create_post;
    pub mod establish_connection;
    pub mod models;
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
        pub mod get_providers_json_local_data;
        pub mod get_providers_link_parts;
        pub mod get_providers_link_parts_from_mongo;
        pub mod get_twitter_providers_names;
    }
    pub mod get_providers_link_parts_wrapper;
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

#[macro_use]
extern crate diesel;

extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use crate::config_mods::config::CONFIG;
use crate::postgres_integration::create_post::create_post;
use crate::postgres_integration::establish_connection::establish_connection;

// use crate::config_mods::config_structs::get_lazy_config_information::TEST;
// use crate::config_mods::config_structs::get_lazy_config_information::TESTTWO;
// use crate::config_mods::env_var_enum::EnvVar;
// use crate::config_mods::env_var_bool_enum::EnvBoolVar;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
fn main() {
    // println!("TEST {:#?}", TESTTWO[&EnvBoolVar::EnableInfoForArxiv]);
    // let bbb =  TEST[&EnvVar::ArxivLink];
    // let f = TESTTWO[&EnvBoolVar::EnableInfoForArxiv];
    entry::entry();
    /////////////////////////////////////////////////////
    let postgres_url = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        CONFIG
            .postgres_params
            .postgres_url_parts
            .postgres_first_handle_url_part,
        CONFIG.postgres_params.postgres_authorization.postgres_login,
        CONFIG
            .postgres_params
            .postgres_url_parts
            .postgres_second_handle_url_part,
        CONFIG
            .postgres_params
            .postgres_authorization
            .postgres_password,
        CONFIG
            .postgres_params
            .postgres_url_parts
            .postgres_third_handle_url_part,
        CONFIG.postgres_params.postgres_authorization.postgres_ip,
        CONFIG
            .postgres_params
            .postgres_url_parts
            .postgres_fourth_handle_url_part,
        CONFIG.postgres_params.postgres_authorization.postgres_port,
        CONFIG
            .postgres_params
            .postgres_url_parts
            .postgres_fifth_handle_url_part,
        CONFIG.postgres_params.postgres_authorization.postgres_db
    );
    let posgtres_connection = establish_connection(postgres_url);
    match posgtres_connection {
        Some(pg_connection) => {
            create_post(&pg_connection, "post_title", "post_body");
        }
        None => {
            println!("todo")
        }
    }
}
