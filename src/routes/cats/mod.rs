//todo - create enum without inner values for returning every possible Http Codes for every route. like 201 or 500
//todo - add check github commit id
//todo - check maybe not need to use everywhere InternalServerError
//todo change methods patch post delete etc
//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
// http://127.0.0.1:8080/api/cats/?limit=87 - Some(87)
//or
// http://127.0.0.1:8080/api/cats/ - None
#[actix_web::get("/")]
pub async fn get(
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::GetQueryParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!(
        "get limit {:?}, name {:?} color {:?}",
        query_parameters.limit, query_parameters.name, query_parameters.color
    );
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => {
            &tufa_common::repositories_types::tufa_server::routes::cats::DEFAULT_SELECT_ALL_LIMIT
        }
    };
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "SELECT * FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&**pool)
            .await
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "SELECT * FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&**pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "SELECT * FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&**pool)
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
            .fetch_all(&**pool)
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
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::GetErrorNamed::PostgresSelect {
                postgres_select: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version()
            ))
        }
    }
}

//http://127.0.0.1:8080/api/cats/756
#[actix_web::get("/{id}")]
pub async fn get_by_id(
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::GetByIdPathParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!("get_by_id {}", path_parameters.id);
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
            error.error_log(**config);
            return actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()));
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_one(&**pool)
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
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()))
        }
    }
}

// curl -X POST http://127.0.0.1:8080/api/cats/ -H 'Content-Type: application/json' -d '{"name":"simba", "color":"black"}'
#[actix_web::post("/")]
pub async fn post(
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToPost>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!("post name {}, color {}", cat.name, cat.color);
    println!("len{}", cat.color.len());
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "INSERT INTO cats(name, color) VALUES ($1, $2)",
        cat.name,
        cat.color
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(_) => actix_web::HttpResponse::Created().finish(),
        Err(e) => {
            // match e {
            //     sqlx::Error::Configuration(box_dyn_error) => todo!(),
            //     sqlx::Error::Database(box_dyn_database_error) => todo!(),
            //     sqlx::Error::Io(io_error) => todo!(),
            //     sqlx::Error::Tls(box_dyn_error) => todo!(),
            //     sqlx::Error::Protocol(string) => todo!(),
            //     sqlx::Error::RowNotFound => todo!(),
            //     sqlx::Error::TypeNotFound { type_name } => todo!(),
            //     sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
            //     sqlx::Error::ColumnNotFound(string) => todo!(),
            //     sqlx::Error::ColumnDecode {
            //         index,
            //         source,
            //     } => todo!(),
            //     sqlx::Error::Decode(box_dyn_error) => todo!(),
            //     sqlx::Error::PoolTimedOut => todo!(),
            //     sqlx::Error::PoolClosed => todo!(),
            //     sqlx::Error::WorkerCrashed => todo!(),
            //     sqlx::Error::Migrate(box_crate_migrate_migrate_error) => todo!(),
            //     _ => todo!()
            // }
            eprintln!("Unable to post a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostErrorNamed::PostgresInsert {
                postgres_insert: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

//todo - its the case if all columns except id are not null. for nullable columns must be different logic
// curl -X PUT http://127.0.0.1:8080/api/cats/ -H 'Content-Type: application/json' -d '{"id": 7, "name":"simba", "color":"black"}'
#[actix_web::put("/")]
pub async fn put(
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::Cat>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!("put id {} name {}, color {}", cat.id, cat.name, cat.color);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(cat.id) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PutErrorNamed::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
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
    .fetch_all(&**pool)
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
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

// curl -X PATCH http://127.0.0.1:8080/api/cats/ -H 'Content-Type: application/json' -d '{"id": 7, "name":"simba"}'
#[actix_web::patch("/")]
pub async fn patch(
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToPatch>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
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
            error.error_log(**config);
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
            error.error_log(**config);
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
            .fetch_all(&**pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                *bigserial_id.bigserial()
            )
            .fetch_all(&**pool)
            .await
        }
        (Some(_), Some(_)) => {
            eprintln!("please use put for full object update");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PatchErrorNamed::PleaseUsePut {
                please_use_put: std::string::String::from("please_use_put"),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
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
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

// curl -X DELETE http://127.0.0.1:8080/api/cats/?color=white
#[actix_web::delete("/")]
pub async fn delete(
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::DeleteQueryParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!("delete name {:?}, color {:?}", query_parameters.name, query_parameters.color);
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            eprintln!("Unable to delete cats, no parameters");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            return actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ));
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "DELETE FROM cats WHERE color = $1",
                color,
            )
            .fetch_all(&**pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "DELETE FROM cats WHERE name = $1",
                name,
            )
            .fetch_all(&**pool)
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "DELETE FROM cats WHERE name = $1 AND color = $2",
                name,
                color
            )
            .fetch_all(&**pool)
            .await
        }
    };
    match query_result {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to delete cats, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteErrorNamed::PostgresDelete {
                postgres_delete: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}

// curl -X DELETE http://127.0.0.1:8080/api/cats/2
#[actix_web::delete("/{id}")]
pub async fn delete_by_id(
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdPathParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
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
            error.error_log(**config);
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
    .fetch_all(&**pool)
    .await {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to delete a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::PostgresDelete {
                postgres_delete: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                error.into_serialize_deserialize_version(),
            ))
        }
    }
}