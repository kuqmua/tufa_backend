pub mod get_project_information {
    pub mod get_names {
        pub mod get_arxiv_names;
        pub mod get_biorxiv_names;
        pub mod get_github_names;
        pub mod get_habr_names;
        pub mod get_medrxiv_names;
        pub mod get_reddit_names;
        pub mod get_twitter_names;
    }
    pub mod generate_hashmap_links {
        pub mod generate_arxiv_hashmap_links;
        pub mod generate_biorxiv_hashmap_links;
        pub mod generate_github_hashmap_links;
        pub mod generate_habr_hashmap_links;
        pub mod generate_medrxiv_hashmap_links;
        pub mod generate_reddit_hashmap_links;
        pub mod generate_twitter_hashmap_links;
    }
    pub mod get_twitter_providers_names;
}

pub mod init_mongo_db_and_collections {
    pub mod get_providers_json_local_data;
    pub mod put_data_in_mongo;
}
