pub(crate) async fn read_by_id(
    path_parameters_extraction_result: Result<
        axum::extract::Path<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdPath,
        >,
        axum::extract::rejection::PathRejection,
    >,
    query_parameters_extraction_result: Result<
        axum::extract::Query<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdQuery,
        >,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdParameters {
        path: match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdPath,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadByIdResponseVariants
        >::try_extract_value(
            path_parameters_extraction_result,
            &app_info_state
        ) {
            Ok(path_parameters) => path_parameters,
            Err(err) => {
                return err;
            },
        },
        query: match tufa_common::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdQuery,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadByIdResponseVariants
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
