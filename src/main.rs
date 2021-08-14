mod fetch {
    // pub mod create_dir_if_dont_exists;
    pub mod async_write_json_into_file;
    pub mod error_logs_into_json;
    pub mod parse_github_html;
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
    pub mod write_error_logs_into_file_for_provider;
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
mod logs_logic {
    pub mod async_write_fetch_error_logs_into_mongo_wrapper;
}
mod check_net {
    pub mod check_link;
    pub mod check_link_metainfo_structures;
    pub mod fetch_link;
}
mod authorization {
    pub mod reddit {
        pub mod reddit_authorization;
    }
}
mod helpers {
    pub mod create_dir_if_dont_exists;
    pub mod json_to_string;
    pub mod write_json_into_file;
}

mod async_tokio_wrapper;
mod check_new_posts_threads_parts;
mod entry;
mod providers_new_posts_check;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
fn main() {
    entry::entry();
}
