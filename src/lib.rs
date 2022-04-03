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
    pub mod create_dir_if_it_doesnt_exist;
    pub mod get_git_commit_info;
    pub mod get_git_commit_string;
    pub mod get_git_source_file_link;
    pub mod get_server_address;
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
mod logs_logic;
pub mod mongo_integration;
pub mod postgres_integration;
pub mod prints;
mod providers;
mod routes;
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
pub mod check_new_providers_posts;
pub mod entry;
pub mod preparation;
pub mod server_wrapper;
pub mod telemetry;
mod traits;
pub mod write_error_posts_wrapper;
