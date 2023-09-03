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
            tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants
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
            tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants
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
    println!("read_by_id parameters {parameters:#?}");
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        parameters.path.id.to_inner()
    )
    .fetch_one(&*app_info_state.get_postgres_pool())
    .await
    {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants::Desirable(value),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id::TryReadByIdResponseVariants::from(error)
        }
    }
}
