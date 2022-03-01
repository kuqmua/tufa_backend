use actix_web::{get, web, Responder};
use std::time::Instant;

use crate::helpers::get_git_source_file_link::get_git_source_file_link;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::providers::get_providers_posts::get_providers_posts;

#[get("/get_providers_posts/")]
async fn get_providers_posts_route() -> impl Responder {
    let time = Instant::now();
    get_providers_posts().await;
    let message = format!("get_providers_posts done in {} seconds", time.elapsed().as_secs());
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        message.clone(),
    );
    message
}