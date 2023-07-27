pub(crate) async fn delete_by_id_axum<'a>(
    axum::extract::Path(path_parameters): axum::extract::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::DeleteByIdPathParameters>,
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants {
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
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error);
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "DELETE FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_all(&*app_info.get_postgres_pool())
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::DesirableType(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error)
        }
    }
}

#[actix_web::delete("/{id}")]
pub(crate) async fn delete_by_id<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::DeleteByIdPathParameters>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
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
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error).into();
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "DELETE FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_all(&app_info.postgres_pool)
    .await
    {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::DesirableType(()).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteByIdResponseVariants::from(error).into()
        }
    }
}