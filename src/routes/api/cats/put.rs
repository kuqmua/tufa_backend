pub async fn put_axum<'a>(
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    axum::Json(payload): axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::Cat>,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants {
    println!("put id {} name {}, color {}", payload.id, payload.name, payload.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        payload.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPut::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::from(error);
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "INSERT INTO cats(id, name, color) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, color = EXCLUDED.color",
        *bigserial_id.bigserial(),
        payload.name,
        payload.color
    )
    .fetch_all(&*app_info.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::DesirableType(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPut::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::from(error)
        }
    }
}

#[actix_web::put("/")]
pub async fn put<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    json: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::Cat>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!("put id {} name {}, color {}", json.id, json.name, json.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        json.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPut::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::from(error).into();
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "INSERT INTO cats(id, name, color) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, color = EXCLUDED.color",
        *bigserial_id.bigserial(),
        json.name,
        json.color
    )
    .fetch_all(&app_info.postgres_pool)
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::DesirableType(()).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPut::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::put::TryPutResponseVariants::from(error).into()
        }
    }
}