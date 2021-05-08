use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::get_project_information::get_config_information::Config;

#[tokio::main]
pub async fn async_tokio_wrapper(config: Config) {
    check_new_posts_threads_parts(config).await;
}
