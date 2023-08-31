pub(crate) async fn put_by_id<'a>(
    path_parameters_extraction_result: Result<
        axum::extract::Path<
            tufa_common::repositories_types::tufa_server::routes::api::cats::PutByIdPathParameters,
        >,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPutById>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let path_parameters = match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::PutByIdPathParameters,
        tufa_common::repositories_types::tufa_server::routes::api::cats::put_by_id::TryPutByIdResponseVariants
    >::try_extract_value(
        path_parameters_extraction_result,
        &app_info_state
    ) {
        Ok(path_parameters) => path_parameters,
        Err(err) => {
            return err;
        },
    };
    let payload = match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPutById,
        tufa_common::repositories_types::tufa_server::routes::api::cats::put_by_id::TryPutByIdResponseVariants
    >::try_extract_value(
        payload_extraction_result,
        &app_info_state
    ) {
        Ok(payload) => payload,
        Err(err) => {
            return err;
        },
    };
    println!("put name {}, color {}", payload.name, payload.color);
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPutById,
        "INSERT INTO cats(id, name, color) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, color = EXCLUDED.color",
        path_parameters.id.to_inner(),
        payload.name,
        payload.color
    )
    .fetch_all(&*app_info_state.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::put_by_id::TryPutByIdResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put_by_id::TryPutById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::put_by_id::TryPutByIdResponseVariants::from(error)
        }
    }
}
