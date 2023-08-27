pub(crate) async fn post_search(
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPost>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    // let payload = match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPost,
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants
    // >::try_extract_value(
    //     payload_extraction_result,
    //     &app_info_state
    // ) {
    //     Ok(payload) => payload,
    //     Err(err) => {
    //         return err;
    //     },
    // };
    // println!("post name {}, color {}", payload.name, payload.color);
    // match sqlx::query_as!(
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
    //     "INSERT INTO cats(name, color) VALUES ($1, $2)",
    //     payload.name,
    //     payload.color
    // )
    // .fetch_all(&*app_info_state.get_postgres_pool())
    // .await
    // {
    //     Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants::Desirable(()),
    //     Err(e) => {
    //         let error = tufa_common::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearch::from(e);
    //         tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
    //             &error,
    //             app_info_state.as_ref(),
    //         );
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::post_search::TryPostSearchResponseVariants::from(error)
    //     }
    // }
    todo!()
}
