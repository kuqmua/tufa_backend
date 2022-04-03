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
