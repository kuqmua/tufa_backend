use crate::threads_parts::threads_parts;

#[tokio::main]
pub async fn tokio_wrapper() {
    threads_parts().await;
}
