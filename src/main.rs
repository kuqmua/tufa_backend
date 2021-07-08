mod fetch {
    pub mod parse_github_html;
    pub mod rss_async_write_fetch_error_logs_into_file;
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
    pub mod rss_logs_create_dir_if_dont_exists;
    pub mod rss_metainfo_fetch_structures;
    pub mod rss_parse_string_into_struct;
    pub mod rss_part;
    pub mod rss_provider_kind_enum;
    pub mod rss_write_error_logs_into_file_for_provider;
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
mod check_net {
    pub mod check_link;
    pub mod check_link_metainfo_structures;
    pub mod fetch_link;
}
mod overriding {
    pub mod prints;
}
mod authorization {
    pub mod reddit {
        pub mod reddit_authorization;
    }
}

mod async_tokio_wrapper;
mod check_new_posts_threads_parts;
mod entry;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::get_user_credentials::get_user_credentials_information::USER_CREDENTIALS;
use providers_info_lib::get_project_information::get_providers_link_parts::get_providers_link_parts;
use providers_info_lib::get_project_information::get_providers_link_parts::Resource;

fn main() {
    let mongo_url: String;
    if CONFIG.mongo_params.is_cloud {
        let mongo_cloud_first_handle_url_part =
            &CONFIG.mongo_params.mongo_cloud_first_handle_url_part;
        let mongo_cloud_login = &USER_CREDENTIALS.mongo_cloud_authorization.mongo_cloud_login;
        let mongo_cloud_second_handle_url_part =
            &CONFIG.mongo_params.mongo_cloud_second_handle_url_part;
        let mongo_cloud_password = &USER_CREDENTIALS
            .mongo_cloud_authorization
            .mongo_cloud_password;
        let mongo_cloud_third_handle_url_part =
            &CONFIG.mongo_params.mongo_cloud_third_handle_url_part;
        let mongo_cloud_cluster_name = &USER_CREDENTIALS
            .mongo_cloud_authorization
            .mongo_cloud_cluster_name;
        let mongo_cloud_fourth_handle_url_part =
            &CONFIG.mongo_params.mongo_cloud_fourth_handle_url_part;
        let mongo_cloud_cluster_params = &USER_CREDENTIALS
            .mongo_cloud_authorization
            .mongo_cloud_cluster_params;
        mongo_url = format!(
            "{}{}{}{}{}{}{}{}",
            mongo_cloud_first_handle_url_part,
            mongo_cloud_login,
            mongo_cloud_second_handle_url_part,
            mongo_cloud_password,
            mongo_cloud_third_handle_url_part,
            mongo_cloud_cluster_name,
            mongo_cloud_fourth_handle_url_part,
            mongo_cloud_cluster_params
        );
    } else {
        let mongo_own_first_handle_url_part = &CONFIG.mongo_params.mongo_own_first_handle_url_part;
        let mongo_own_login = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_login;
        let mongo_own_second_handle_url_part =
            &CONFIG.mongo_params.mongo_own_second_handle_url_part;
        let mongo_own_password = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_password;
        let mongo_own_third_handle_url_part = &CONFIG.mongo_params.mongo_own_third_handle_url_part;
        let mongo_own_ip = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_ip;
        let mongo_own_fourth_handle_url_part =
            &CONFIG.mongo_params.mongo_own_fourth_handle_url_part;
        let mongo_own_port = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_port;
        mongo_url = format!(
            "{}{}{}{}{}{}{}{}",
            mongo_own_first_handle_url_part,
            mongo_own_login,
            mongo_own_second_handle_url_part,
            mongo_own_password,
            mongo_own_third_handle_url_part,
            mongo_own_ip,
            mongo_own_fourth_handle_url_part,
            mongo_own_port
        );
    }
    // put_data_in_mongo(
    //     &mongo_url,
    //     &CONFIG.mongo_params.db_name_handle,
    //     &CONFIG.mongo_params.db_collection_handle_second_part,
    //     &CONFIG.mongo_params.db_collection_document_field_name_handle,
    //     &CONFIG.mongo_params.path_to_provider_link_parts_folder,
    //     CONFIG.mongo_params.vec_of_provider_names.clone(),
    //     &CONFIG.mongo_params.file_extension,
    // );
    // entry::entry();
    let providers_link_parts = get_providers_link_parts(&Resource::Mongodb);

    // println!("providers_link_parts {:#?}", providers_link_parts)
}
