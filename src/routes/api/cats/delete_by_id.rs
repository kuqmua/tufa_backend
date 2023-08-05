pub(crate) async fn delete_by_id<'a>(
    path_parameters_extraction_result: Result<
        axum::extract::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPathParameters>,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let path_parameters = match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPathParameters,
        tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants
    >::try_extract_value(
        path_parameters_extraction_result,
        &app_info_state
    ) {
        Ok(path_parameters) => path_parameters,
        Err(err) => {
            return err;
        },
    };
    println!("delete_by_id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById::Bigserial {
                bigserial: e,
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLogSecond::error_log_second(
                &error,
                &app_info_state.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error);
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "DELETE FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_all(&*app_info_state.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLogSecond::error_log_second(
                &error,
                &app_info_state.get_config()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error)
        }
    }
}
