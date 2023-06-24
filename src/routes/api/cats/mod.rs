//todo - create enum without inner values for returning every possible Http Codes for every route. like 201 or 500
//todo - check maybe not need to use everywhere InternalServerError
//todo change methods patch post delete etc
//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
//todo - maybe add link to function API usage and name of function to use instead and send it 
//todo - wrap route logic to function what return Result. after match result and return actix_web::HttpResponse
//todo - add limit everywhere possible
//// request: actix_web::HttpRequest,
//todo find out how to create middleware without extractors
// curl -X GET http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615&limit=87 - Some(87)
//or
// curl -X GET http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 - None
//todo - change curl example - not always works with query params
// http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615&name=leo&color=red
#[actix_web::get("/")]
pub async fn get<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> actix_web::HttpResponse {
    println!(
        "get query_parameters limit {:?}, name {:?} color {:?}",
        query_parameters.limit, query_parameters.name, query_parameters.color
    );
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => {
            &tufa_common::server::postgres::constants::DEFAULT_POSTGRES_SELECT_LIMIT
        }
    };
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
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
        Ok(vec_cats) => actix_web::HttpResponse::Ok().json(actix_web::web::Json(vec_cats)),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get::route::GetErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get::route::GetHttpResponse::from(error).into()
        }
    }
}

// curl -X GET http://127.0.0.1:8080/api/cats/7?project_commit=18446744073709551615
#[actix_web::get("/{id}")]
pub async fn get_by_id<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            return error.into();
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
        Ok(cat) => {
            // tracing::info!("selected casts:\n{vec_cats:#?}");
            println!("selected cats:\n{cat:#?}");
            actix_web::HttpResponse::Ok().json(actix_web::web::Json(cat))
        }
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            error.into()
        }
    }
}

// curl -X POST http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 -H 'Content-Type: application/json' -d '{"name":"simba", "color":"black"}'
#[actix_web::post("/")]
pub async fn post<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::post::CatToPost>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    println!("post name {}, color {}", cat.name, cat.color);
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "INSERT INTO cats(name, color) VALUES ($1, $2)",
        cat.name,
        cat.color
    )
    .fetch_all(&app_info.postgres_pool)
    .await
    {
        Ok(_) => actix_web::HttpResponse::Created().finish(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::post::route::PostErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            error.into()
        }
    }
}

//todo - its the case if all columns except id are not null. for nullable columns must be different logic
// curl -X PUT http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 -H 'Content-Type: application/json' -d '{"id": 7, "name":"simba", "color":"black"}'
#[actix_web::put("/")]
pub async fn put<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::Cat>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    println!("put id {} name {}, color {}", cat.id, cat.name, cat.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(cat.id) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::route::PutErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            return error.into();
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "INSERT INTO cats(id, name, color) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, color = EXCLUDED.color",
        *bigserial_id.bigserial(),
        cat.name,
        cat.color
    )
    .fetch_all(&app_info.postgres_pool)
    .await
    {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::put::route::PutErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            error.into()
        }
    }
}

// curl -X PATCH http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 -H 'Content-Type: application/json' -d '{"id": 7, "name":"simba"}'
#[actix_web::patch("/")]
pub async fn patch<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    println!("patch cat {cat:#?}");
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        *cat.get_id(),
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch::route::PatchErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            return error.into()
        }
    };
    let query_result = match &*cat {
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
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch::route::PatchErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            error.into()
        }
    }
}

// curl -X DELETE http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615&color=white
#[actix_web::delete("/")]
pub async fn delete<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::api::cats::delete::DeleteQueryParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    println!("delete name {:?}, color {:?}", query_parameters.name, query_parameters.color);
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::route::DeleteErrorNamed::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            return error.into()
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE color = $1",
                color,
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE name = $1",
                name,
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE name = $1 AND color = $2",
                name,
                color
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
    };
    match query_result {
        Ok(_) => {
            println!("ok delete");
            actix_web::HttpResponse::Ok().finish()
        },
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::route::DeleteErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            error.into()
        }
    }
}

// curl -X DELETE http://127.0.0.1:8080/api/cats/14?project_commit=36cd5a29d00ddbcfc32ebcaad76cc63696fdc0e5
#[actix_web::delete("/{id}")]
pub async fn delete_by_id<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::DeleteByIdPathParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    println!("delete_by_id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::route::DeleteByIdErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            return error.into();
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "DELETE FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_all(&app_info.postgres_pool)
    .await {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::route::DeleteByIdErrorNamed::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            error.into()
        }
    }
}