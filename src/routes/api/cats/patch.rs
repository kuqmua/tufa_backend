pub(crate) async fn patch<'a>(
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch,
        >,
        axum::extract::rejection::JsonRejection,
    >
) -> impl axum::response::IntoResponse {
    let axum::Json(payload) = match payload_extraction_result {
        Ok(payload) => payload,
        Err(err) => {
            let error = tufa_common::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed::from(err);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::from(error);
        }
    };
    println!("patch {payload:#?}");
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        *tufa_common::server::postgres::get_postgres_bigserial_id::GetPostgresBigserialId::get_postgres_bigserial_id(&payload),
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatch::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::from(error);
        }
    };
    let query_result = match payload {
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::IdName { id: _id, name } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                *bigserial_id.bigserial()
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::IdColor { id: _id, color } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET color = $1 WHERE id = $2",
                color,
                *bigserial_id.bigserial()
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        },
    };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatch::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::from(error)
        }
    }
}