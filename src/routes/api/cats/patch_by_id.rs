pub(crate) async fn patch_by_id<'a>(
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let payload = match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch,
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants
    >::try_extract_value(
        payload_extraction_result,
        &app_info_state
    ) {
        Ok(payload) => payload,
        Err(err) => {
            return err;
        },
    };
    println!("patch {payload:#?}");
    let bigserial_id = *tufa_common::server::postgres::bigserial::GetPostgresBigserialId::get_postgres_bigserial_id(&payload).to_inner();
    let query_result = match payload {
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::IdName {
            id: _id,
            name,
        } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                bigserial_id
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::IdColor {
            id: _id,
            color,
        } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET color = $1 WHERE id = $2",
                color,
                bigserial_id
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        }
    };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants::from(error)
        }
    }
}
