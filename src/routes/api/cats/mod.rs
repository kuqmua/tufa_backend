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
//todo header Retry-After logic
#[actix_web::get("/")]
pub async fn get<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    query_parameters: actix_web::web::Query<
        tufa_common::repositories_types::tufa_server::routes::api::cats::get::GetQueryParameters,
    >,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
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
        Ok(vec_cats) => tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::DesirableType(vec_cats).into(),
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

#[actix_web::get("/{id}")]
pub async fn get_by_id<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(path_parameters.id).unwrap();
    // let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
    //     path_parameters.id,
    // ) {
    //     Ok(bigserial_id) => bigserial_id,
    //     Err(e) => {
    //         let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdErrorNamed::Bigserial { 
    //             bigserial: e, 
    //             code_occurence: tufa_common::code_occurence!()
    //         };
    //         tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
    //             &error,
    //             &app_info.config,
    //         );
    //         return tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::route::GetByIdHttpResponse::from(error).into();
    //     }
    // };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_one(&app_info.postgres_pool)
    .await
    {
        Ok(cat) => {
            println!("selected cats:\n{cat:#?}");
            tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::DesirableType(cat).into()
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

#[actix_web::post("/")]
pub async fn post<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    cat: actix_web::web::Json<
        tufa_common::repositories_types::tufa_server::routes::api::cats::post::CatToPost,
    >,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
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
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPostResponseVariants::DesirableType(()).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPost::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::post::TryPostResponseVariants::from(error).into()
        }
    }
}

//todo - its the case if all columns except id are not null. for nullable columns must be different logic
#[actix_web::put("/")]
pub async fn put<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::Cat>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!("put id {} name {}, color {}", cat.id, cat.name, cat.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        cat.id,
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
        cat.name,
        cat.color
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

#[actix_web::patch("/")]
pub async fn patch<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    cat: actix_web::web::Json<
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch::CatToPatch,
    >,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!("patch cat {cat:#?}");
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        *cat.get_id(),
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

#[actix_web::delete("/")]
pub async fn delete<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::api::cats::delete::DeleteQueryParameters>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!(
        "delete name {:?}, color {:?}",
        query_parameters.name, query_parameters.color
    );
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error).into();
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
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::DesirableType(()).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error).into()
        }
    }
}

#[actix_web::delete("/{id}")]
pub async fn delete_by_id<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::DeleteByIdPathParameters>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>,
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
