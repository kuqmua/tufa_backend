pub async fn get_by_id_axum(
    axum::extract::Path(path_parameters): axum::extract::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters>,
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants {
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error);
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_one(&*app_info.get_postgres_pool())
    .await
    {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::DesirableType(value),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error)
        }
    }
}

#[actix_web::get("/{id}")]
pub async fn get_by_id<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error).into();
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_one(&app_info.postgres_pool)
    .await
    {
        Ok(value) => {
            println!("selected cats:\n{value:#?}");
            tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::DesirableType(value).into()
        }
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error).into()
        }
    }
}