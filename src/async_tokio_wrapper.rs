use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::get_project_information::get_config::config_structures::ConfigStruct;

#[tokio::main]
pub async fn async_tokio_wrapper(config: ConfigStruct) {
    check_new_posts_threads_parts(config).await;
}
