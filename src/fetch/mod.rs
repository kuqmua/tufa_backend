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
