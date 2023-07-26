pub async fn get_axum(
    axum::extract::Query(query_parameters): axum::extract::Query<tufa_common::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters>,
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants {
    println!(
        "get query_parameters limit {:?}, name {:?} color {:?}",
        query_parameters.limit, query_parameters.name, query_parameters.color
    );
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => &tufa_common::server::postgres::constants::DEFAULT_POSTGRES_SELECT_LIMIT,
    };
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            //todo make rust executable for calling cargo run + DATABASE_URL="" - then can easy move it into lib (tufa_common)
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
    };
    match query_result {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::DesirableType(value),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGet::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error)
        }
    }
}

#[actix_web::get("/")]
pub async fn get<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    query_parameters: actix_web::web::Query<
        tufa_common::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters,
    >,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!(
        "get query_parameters limit {:?}, name {:?} color {:?}",
        query_parameters.limit, query_parameters.name, query_parameters.color
    );
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => &tufa_common::server::postgres::constants::DEFAULT_POSTGRES_SELECT_LIMIT,
    };
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            //todo make rust executable for calling cargo run + DATABASE_URL="" - then can easy move it into lib (tufa_common)
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT * FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
    };
    match query_result {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::DesirableType(value).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGet::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error).into()
        }
    }
}