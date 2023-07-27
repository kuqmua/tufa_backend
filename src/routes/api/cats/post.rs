pub(crate) async fn post_axum(
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    //write middleware to check if conent type is application\json. return error if its not.
    //use body: string here. serde::from_json later as variant of TryPost
    axum::Json(payload): axum::Json<
        tufa_common::repositories_types::tufa_server::routes::api::cats::post::CatToPost,
    >,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPostResponseVariants
{
    println!("post name {}, color {}", payload.name, payload.color);
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "INSERT INTO cats(name, color) VALUES ($1, $2)",
        payload.name,
        payload.color
    )
    .fetch_all(&*app_info.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPostResponseVariants::DesirableType(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPost::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPostResponseVariants::from(error)
        }
    }
}
