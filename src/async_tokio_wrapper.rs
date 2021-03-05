use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

#[tokio::main]
pub async fn tokio_wrapper() {
    check_new_posts_threads_parts().await;
}
