// #[tracing::instrument(
//     name = "get_providers_posts_routee",
//     skip_all,
//     fields(user_id=%*user_id)
// )]
pub async fn get_providers_posts_route() -> Result<actix_web::HttpResponse, actix_web::Error> {
    let time = std::time::Instant::now();
    if let Err(e) = tufa_common::repositories_types::tufa_server::providers::get_providers_posts::get_providers_posts(
        {
            use std::ops::Deref;
            crate::global_variables::runtime::config::CONFIG.deref()
        }
    ).await {
        return Ok(actix_web::HttpResponse::InternalServerError().finish());
    };
    let message = format!(
        "get_providers_posts done in {} seconds",
        time.elapsed().as_secs()
    );
    Ok(actix_web::HttpResponse::Ok().finish())
}
