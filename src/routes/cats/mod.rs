//todo - create enum without inner values for returning every possible Http Codes for every route. like 201 or 500
//todo - check maybe not need to use everywhere InternalServerError
//todo change methods patch post delete etc
//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
//todo - maybe add link to function API usage and name of function to use instead and send it 
//todo - wrap route logic to function what return Result. after match result and return actix_web::HttpResponse
// curl -X GET http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615&limit=87 - Some(87)
//or
// curl -X GET http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 - None
//todo - change curl example - not always works with query params
// http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615&name=leo&color=red
#[actix_web::get("/")]
pub async fn get<'a>(
    request: actix_web::HttpRequest,
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::GetQueryParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::GetErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::GetErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::GetErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
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
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "SELECT * FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "SELECT * FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "SELECT * FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
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
        Ok(vec_cats) => {
            // tracing::info!("selected casts:\n{vec_cats:#?}");
            println!("get casts:\n{vec_cats:#?}");
            actix_web::HttpResponse::Ok().json(actix_web::web::Json(vec_cats))
        }
        Err(e) => {
            // todo https://github.com/cschaible/actix-web-security-samples/blob/46bb7aa62ada7cb176d8765e2f60b497392b1840/oauth-resource-server/backend/src/error/mod.rs#L46
            // todo https://www.postgresql.org/docs/current/errcodes-appendix.html
            // match e {
            //     sqlx::Error::Configuration(box_dyn_error) => println!(""),
            //     sqlx::Error::Database(box_dyn_database_error) => {
            //         println!("");
            //         match box_dyn_database_error.code() {

            //         }
            //     },
            //     sqlx::Error::Io(io_error) => println!(""),
            //     sqlx::Error::Tls(box_dyn_error) => println!(""),
            //     sqlx::Error::Protocol(string) => println!(""),
            //     sqlx::Error::RowNotFound => println!(""),
            //     sqlx::Error::TypeNotFound { type_name } => println!(""),
            //     sqlx::Error::ColumnIndexOutOfBounds { index, len } => println!(""),
            //     sqlx::Error::ColumnNotFound(string) => println!(""),
            //     sqlx::Error::ColumnDecode {
            //         index,
            //         source,
            //     } => println!(""),
            //     sqlx::Error::Decode(box_dyn_error) => println!(""),
            //     sqlx::Error::PoolTimedOut => println!(""),
            //     sqlx::Error::PoolClosed => println!(""),
            //     sqlx::Error::WorkerCrashed => println!(""),
            //     sqlx::Error::Migrate(box_crate_migrate_migrate_error) => println!(""),
            //     _ => println!("")
            // }
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::GetErrorNamed::PostgresSelect {
                postgres_select: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ))
        }
    }
}

// curl -X GET http://127.0.0.1:8080/api/cats/7?project_commit=18446744073709551615
#[actix_web::get("/{id}")]
pub async fn get_by_id<'a>(
    request: actix_web::HttpRequest,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::GetByIdPathParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::GetByIdErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::GetByIdErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::GetByIdErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::GetByIdErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()));
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
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
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::GetByIdErrorNamed::PostgresSelect { 
                postgres_select: e, 
                code_occurence: tufa_common::code_occurence!() 
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()))
        }
    }
}

// curl -X POST http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 -H 'Content-Type: application/json' -d '{"name":"simba", "color":"black"}'
#[actix_web::post("/")]
pub async fn post<'a>(
    request: actix_web::HttpRequest,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToPost>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::PostErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::PostErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
    println!("post name {}, color {}", cat.name, cat.color);
    println!("len{}", cat.color.len());
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "INSERT INTO cats(name, color) VALUES ($1, $2)",
        cat.name,
        cat.color
    )
    .fetch_all(&app_info.postgres_pool)
    .await
    {
        Ok(_) => actix_web::HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Unable to post a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostErrorNamed::PostgresInsert {
                postgres_insert: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

//todo - its the case if all columns except id are not null. for nullable columns must be different logic
// curl -X PUT http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 -H 'Content-Type: application/json' -d '{"id": 7, "name":"simba", "color":"black"}'
#[actix_web::put("/")]
pub async fn put<'a>(
    request: actix_web::HttpRequest,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::Cat>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::PutErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::PutErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PutErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
    println!("put id {} name {}, color {}", cat.id, cat.name, cat.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(cat.id) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PutErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()));
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
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
            eprintln!("Unable to put a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PutErrorNamed::PostgresInsertOrUpdate {
                postgres_insert_or_update: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

// curl -X PATCH http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615 -H 'Content-Type: application/json' -d '{"id": 7, "name":"simba"}'
#[actix_web::patch("/")]
pub async fn patch<'a>(
    request: actix_web::HttpRequest,
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToPatch>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
    println!("patch name {:?}, color {:?}", cat.name, cat.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        cat.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                )
            );
        }
    };
    let query_result = match (&cat.name, &cat.color) {
        (None, None) => {
            eprintln!("Unable to patch a cat, no parameters");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ));
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "UPDATE cats SET color = $1 WHERE id = $2",
                color,
                *bigserial_id.bigserial()
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                *bigserial_id.bigserial()
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(_), Some(_)) => {
            eprintln!("please use put for full object update");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::PleaseUsePut {
                please_use_put: std::string::String::from("please_use_put"),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ));
        }
    };
    match query_result {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to patch a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::PostgresUpdate {
                postgres_update: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

// curl -X DELETE http://127.0.0.1:8080/api/cats/?project_commit=18446744073709551615&color=white
#[actix_web::delete("/")]
pub async fn delete<'a>(
    request: actix_web::HttpRequest,
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::DeleteQueryParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
    println!("delete name {:?}, color {:?}", query_parameters.name, query_parameters.color);
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            eprintln!("Unable to delete cats, no parameters");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            let json = actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            );
            println!("{json}");
            return actix_web::HttpResponse::InternalServerError().json(json);
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "DELETE FROM cats WHERE color = $1",
                color,
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "DELETE FROM cats WHERE name = $1",
                name,
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
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
            eprintln!("Unable to delete cats, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::PostgresDelete {
                postgres_delete: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

// curl -X DELETE http://127.0.0.1:8080/api/cats/14?project_commit=36cd5a29d00ddbcfc32ebcaad76cc63696fdc0e5
#[actix_web::delete("/{id}")]
pub async fn delete_by_id<'a>(
    request: actix_web::HttpRequest,
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdPathParameters>,
    app_info: actix_web::web::Data<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'a>>,
) -> impl actix_web::Responder {
    match request.headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        Some(project_commit_header_value) => match project_commit_header_value.to_str() {
            Ok(possible_project_commit) => {
                if let true = possible_project_commit != app_info.project_git_info.project_commit {
                    let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::CheckApiUsage {
                        project_commit: app_info.project_git_info.does_not_match_message(),
                        code_occurence: tufa_common::code_occurence!(),
                    };
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    error.error_log(app_info.config);
                    return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                        error.into_serialize_deserialize_version()
                    ));
                }
            },
            Err(e) => {
                let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::CannotConvertProjectCommitToStr {
                    cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
                    code_occurence: tufa_common::code_occurence!(),
                };
                use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                error.error_log(app_info.config);
                return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                ));
            }
        },
        None => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::NoProjectCommitHeader {
                no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ));
        }
    };
    println!("delete_by_id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            return actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    error.into_serialize_deserialize_version()
                )
            );
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "DELETE FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_all(&app_info.postgres_pool)
    .await {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to delete a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::PostgresDelete {
                postgres_delete: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(app_info.config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}