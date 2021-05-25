use std::thread;

use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use futures::executor::block_on;

use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;

#[tokio::main]
pub async fn async_tokio_wrapper() {
    let (_posts, error_posts) = check_new_posts_threads_parts().await;
    if !error_posts.is_empty() {
        let wrong_cases_thread = thread::spawn(move || {
            // println!("error_posts_done_len{:#?}", error_posts);
            block_on(rss_async_write_fetch_error_logs_into_files_wrapper(
                error_posts,
            ));
        });
        wrong_cases_thread.join().unwrap();
    }
}
