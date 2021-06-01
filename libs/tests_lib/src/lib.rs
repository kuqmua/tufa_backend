mod get_project_information {
    // pub mod get_names {
    //     pub mod get_arxiv_names;
    //     pub mod get_biorxiv_names;
    //     pub mod get_github_names;
    //     pub mod get_habr_names;
    //     pub mod get_medrxiv_names;
    //     pub mod get_reddit_names;
    //     pub mod get_twitter_names;
    // }
    // pub mod generate_hashmap_links {
    //     pub mod generate_arxiv_hashmap_links;
    //     pub mod generate_biorxiv_hashmap_links;
    //     pub mod generate_github_hashmap_links;
    //     pub mod generate_habr_hashmap_links;
    //     pub mod generate_medrxiv_hashmap_links;
    //     pub mod generate_reddit_hashmap_links;
    //     pub mod generate_twitter_hashmap_links;
    // }
    pub mod get_config {
        pub mod config_structures;
        pub mod get_config_information;
    }
    pub mod get_user_credentials {
        pub mod get_user_credentials_information;
        pub mod user_credentials_structures;
    }
    // pub mod get_twitter_providers_names;
    pub mod project_constants;
}

#[cfg(test)]
mod tests {
    pub mod continuous_integration {
        pub mod ci_check_compromised_reddit_auth_info;
        pub mod ci_check_config_files_exists;
    }
    mod tests_constants;
}

#[macro_use]
extern crate lazy_static;
