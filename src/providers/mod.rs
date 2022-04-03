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
pub mod provider_kind {
    pub mod functions {
        pub mod fetch_and_parse_provider_data;
        pub mod get_link_parts_from_local_json_file;
        pub mod get_mongo_provider_link_parts_aggregation;
        pub mod is_prints_for_print_type_enabled;
        pub mod mongo_get_provider_link_parts;
        pub mod rss_part;
    }
    pub mod provider_kind_enum;
    pub mod provider_kind_trait;
}
pub mod check_providers_link_parts_on_empty;
pub mod get_providers_posts;
