pub(crate) async fn put<'a>(
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPut>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let payload = match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPut,
        tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants
    >::try_extract_value(
        payload_extraction_result,
        &app_info_state
    ) {
        Ok(payload) => payload,
        Err(err) => {
            return err;
        },
    };
    println!(
        "put id {} name {}, color {}",
        payload.id, payload.name, payload.color
    );
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPut,
        "INSERT INTO cats(id, name, color) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, color = EXCLUDED.color",
        payload.id.to_inner(),
        payload.name,
        payload.color
    )
    .fetch_all(&*app_info_state.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPut::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::from(error)
        }
    }
}
