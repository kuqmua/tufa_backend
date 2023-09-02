pub(crate) async fn read(
    query_parameters_extraction_result: Result<
        axum::extract::Query<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadQueryParameters,
        >,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let query_parameters = match tufa_common::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadQueryParameters,
        tufa_common::repositories_types::tufa_server::routes::api::cats::read::TryReadResponseVariants
    >::try_extract_value(
        query_parameters_extraction_result,
        &app_info_state
    ) {
        Ok(query_parameters) => query_parameters,
        Err(err) => {
            return err;
        },
    };
    query_parameters.execute_query(&app_info_state).await
}
