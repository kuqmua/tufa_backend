pub(crate) async fn delete_post<'a>(
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<
            tufa_common::repositories_types::tufa_server::routes::api::cats::DeletePostPayload,
        >,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::DeletePostParameters {
        payload: match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::DeletePostPayload,
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete_post::TryDeletePostResponseVariants
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
    println!("{parameters:#?}");
    parameters.prepare_and_execute_query(&app_info_state).await
}
