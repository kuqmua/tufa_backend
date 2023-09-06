pub(crate) async fn read_post(
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadPostPayload,
        >,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::ReadPostParameters {
        payload: match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
            tufa_common::repositories_types::tufa_server::routes::api::cats::ReadPostPayload,
            tufa_common::repositories_types::tufa_server::routes::api::cats::read_post::TryReadPostResponseVariants
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
    parameters.execute_query(&app_info_state).await
}
