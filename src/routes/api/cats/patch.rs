pub(crate) async fn patch_axum<'a>(
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    axum::Json(payload): axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch>,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants {
    println!("patch {payload:#?}");
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        *payload.get_id(),
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
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch::IdName { id: _id, name } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                *bigserial_id.bigserial()
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch::IdColor { id: _id, color } => {
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
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::DesirableType(()),
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

#[actix_web::patch("/")]
pub(crate) async fn patch<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    json: actix_web::web::Json<
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch,
    >,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!("patch {json:#?}");
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        *json.get_id(),
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatch::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::from(error).into();
        }
    };
    let query_result = match &*json {
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch::IdName { id: _id, name } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                *bigserial_id.bigserial()
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch::IdColor { id: _id, color } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET color = $1 WHERE id = $2",
                color,
                *bigserial_id.bigserial()
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        },
    };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::DesirableType(()).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatch::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::patch::TryPatchResponseVariants::from(error).into()
        }
    }
}