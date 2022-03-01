mod check_net {
    pub mod check_net_availability;
    pub mod check_net_enum;
    pub mod check_net_wrapper;
    pub mod check_status_code;
}
mod constants {
    pub mod project_constants;
}
mod fetch {
    pub mod parse_github_html;
    pub mod rss_async_write_fetch_error_logs_into_files_wrapper;
    pub mod rss_handle_error_status_code;
    pub mod rss_metainfo_fetch_structures;
    pub mod rss_parse_string_into_struct;
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
    pub mod config_functions {
        pub mod check_valid_i64_providers_links_limits_for_mongo;
        pub mod wrap_config_checks; //stay
    }
    pub mod config_struct;
    pub mod lazy_static_config;
}
pub mod helpers {
    pub mod get_server_address;
    pub mod create_dir_if_it_doesnt_exist;
    pub mod get_git_commit_info;
    pub mod get_git_commit_string;
    pub mod get_git_source_file_link;
    pub mod lazy_static_git_info;
    pub mod resource;
    pub mod where_was;
    pub mod write_json_into_file;
    pub mod write_string_into_file;
    pub mod write_string_into_file_with_tokio;
    pub mod fetch {
        pub mod async_fetch_link;
        pub mod blocking_fetch_link;
        pub mod fetch_link_error;
    }
}
pub mod init_dbs_logic {
    pub mod dbs_enum;
    pub mod init_dbs;
    pub mod init_dbs_with_providers_link_parts;
    pub mod init_mongo;
    pub mod init_postgres;
    pub mod init_tables_enum;
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
    pub mod mongo_get_providers_link_parts;
    pub mod mongo_insert_data;
    pub mod mongo_insert_docs_in_empty_collection;
}
pub mod postgres_integration {
    pub mod postgres_check_availability;
    pub mod postgres_check_providers_link_parts_tables_are_empty;
    pub mod postgres_check_providers_links_tables_length_rows_equal_initialization_data_length;
    pub mod postgres_create_providers_tables_if_not_exists;
    pub mod postgres_delete_all_from_providers_link_parts_tables;
    pub mod postgres_get_db_url;
    pub mod postgres_get_providers_link_parts;
    pub mod postgres_insert_link_parts_into_providers_tables;
}
pub mod prints {
    pub mod print_colorful_message;
    pub mod print_type_enum;
    pub mod print_wrapper;
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
        pub mod get_local_providers_link_parts;
        pub mod get_providers_link_parts;
        pub mod get_twitter_provider_name;
        pub mod providers_init_json_schema;
    }
    pub mod provider_kind_impl {
        pub mod functions {
            pub mod fetch_and_parse_provider_data;
            pub mod get_link_parts_from_local_json_file;
            pub mod get_mongo_provider_link_parts_aggregation;
            pub mod is_prints_for_print_type_enabled;
            pub mod mongo_get_provider_link_parts;
            pub mod rss_part;
        }
        pub mod provider_kind_trait;
    }
    pub mod check_providers_link_parts_on_empty;
    pub mod get_providers_posts;
    pub mod provider_kind_enum;
}
mod routes {
    pub mod post_echo;
    pub mod hello;
    pub mod index;
    pub mod kekw;
    pub mod manual_responder;
}
#[cfg(test)]
mod tests {
    pub mod tests_constants;
    pub mod continuous_integration {
        pub mod ci_check_compromised_env_vars;
        pub mod ci_check_docker_compose_file_exists;
        pub mod ci_check_env_file_exists;
        pub mod ci_check_env_var_names_contains_in_docker_compose;
    }
}
mod traits {
    pub mod git_info_trait;
    pub mod print_type_trait;
    pub mod provider_kind_from_config_trait;
    pub mod provider_kind_trait;
    pub mod wrap_config_checks_trait;
}
mod check_new_posts_threads_parts;
mod entry;
mod write_error_posts_wrapper;
mod server_wrapper;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate box_err_from_err;
#[macro_use]
extern crate enum_extention;
#[macro_use]
extern crate error_display;
#[macro_use]
extern crate gen_enum;
#[macro_use]
extern crate gen_enum_without_values;
#[macro_use]
extern crate git_info;
#[macro_use]
extern crate impl_display;
#[macro_use]
extern crate impl_from_for_upper_struct;
#[macro_use]
extern crate init_from_env;
#[macro_use]
extern crate provider_kind_from_config;

extern crate dotenv;

use crate::entry::entry;

fn main() {
    entry();
}