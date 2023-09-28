pub(crate) async fn delete<'a>(
    query_parameters_extraction_result: Result<
        axum::extract::Query<
            tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteQuery,
        >,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteParameters {
        query: match tufa_common::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteQuery,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteResponseVariants
        >::try_extract_value(
            query_parameters_extraction_result,
            &app_info_state
        ) {
            Ok(query_parameters) => query_parameters,
            Err(err) => {
                return err;
            },
        },
    };
    println!("{parameters:#?}");
    parameters.prepare_and_execute_query(&app_info_state).await
}
