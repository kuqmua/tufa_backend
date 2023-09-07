pub(crate) async fn create_or_update_by_id<'a>(
    path_parameters_extraction_result: Result<
        axum::extract::Path<
            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOrUpdateByIdPath,
        >,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<
            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOrUpdateByIdPayload,
        >,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOrUpdateByIdParameters {
        path: match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOrUpdateByIdPath,
            tufa_common::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants
        >::try_extract_value(
            path_parameters_extraction_result,
            &app_info_state
        ) {
            Ok(path_parameters) => path_parameters,
            Err(err) => {
                return err;
            },
        },
        payload: match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOrUpdateByIdPayload,
            tufa_common::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::TryCreateOrUpdateByIdResponseVariants
        >::try_extract_value(
            payload_extraction_result,
            &app_info_state
        ) {
            Ok(payload) => payload,
            Err(err) => {
                return err;
            },
        },
    };
    parameters.prepare_and_execute_query(&app_info_state).await
}
