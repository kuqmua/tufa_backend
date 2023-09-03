pub(crate) async fn delete_by_id<'a>(
    path_parameters_extraction_result: Result<
        axum::extract::Path<
            tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPath,
        >,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdParameters {
        path: match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPath,
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants
        >::try_extract_value(
            path_parameters_extraction_result,
            &app_info_state
        ) {
            Ok(path_parameters) => path_parameters,
            Err(err) => {
                return err;
            },
        },
    };
    println!("{parameters:#?}");
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "DELETE FROM cats WHERE id = $1",
        parameters.path.id.to_inner()
    )
    .fetch_all(&*app_info_state.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error)
        }
    }
}
