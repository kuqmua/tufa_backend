// http://127.0.0.1:8080/api/cats/?limit=87 - Some(87)
//or
// http://127.0.0.1:8080/api/cats/ - None
#[actix_web::get("/")]
pub async fn select(
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::SelectQueryParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - add check github commit id
) -> impl actix_web::Responder {
    println!(
        "select, limit {:?}, name {:?} color {:?}",
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
            println!("selected casts:\n{vec_cats:#?}");
            actix_web::HttpResponse::Ok().json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::GetAllResponse::Ok(
                    vec_cats,
                ),
            ))
        }
        Err(e) => {
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresSelectCatsErrorNamed::SelectCats {
                select_cats: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError().json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::GetAllResponse::Err(
                    error.into_serialize_deserialize_version(),
                ),
            ))
        }
    }
}
//http://127.0.0.1:8080/api/cats/756
#[actix_web::get("/{id}")]
pub async fn select_by_id(
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdPathParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!("select_by_id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdErrorNamed::Bigserial { 
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
            let error = tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdErrorNamed::SelectCat { 
                select_cat: e, 
                code_occurence: tufa_common::code_occurence!() 
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()))
        }
    }
}
//todo change methods patch post delete etc
// curl -X POST http://127.0.0.1:8080/api/cats/ -H 'Content-Type: application/json' -d '{"name":"simba", "color":"black"}'
#[actix_web::post("/insert_one")]
pub async fn insert_one(
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToInsert>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> impl actix_web::Responder {
    println!("insert one name {}, color {}", cat.name, cat.color);
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "INSERT INTO cats(name, color) VALUES ($1, $2)",
        cat.name,
        cat.color
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to insert a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresInsertOneErrorNamed::Insert {
                insert: e,
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

// curl -X POST http://127.0.0.1:8080/api/cats/update_one -H 'Content-Type: application/json' -d '{"id": 6, "name":"simba"}'
#[actix_web::post("/update_one")]
pub async fn update_one(
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToUpdate>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - check maybe not need to use everywhere InternalServerError
) -> impl actix_web::Responder {
    //todo how to handle sql injection ?
    println!("update one name {:?}, color {:?}", cat.name, cat.color);
    //
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        cat.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOneErrorNamed::Bigserial { 
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
            eprintln!("Unable to update a cat, no parameters");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOneErrorNamed::NoParameters {
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
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "UPDATE cats SET name = $1, color = $2 WHERE id = $3",
                name,
                color,
                *bigserial_id.bigserial()
            )
            .fetch_all(&**pool)
            .await
        }
    };
    match query_result {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to update a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOneErrorNamed::Update {
                update: e,
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

#[actix_web::patch("/update_one_patch")]
pub async fn update_one_patch(
    cat: actix_web::web::Json<tufa_common::repositories_types::tufa_server::routes::cats::CatToPatch>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - check maybe not need to use everywhere InternalServerError
) -> impl actix_web::Responder {
    //todo how to handle sql injection ?
    println!("update one patch name {:?}, color {:?}", cat.name, cat.color);
    //
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        cat.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOnePatchErrorNamed::Bigserial { 
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
            eprintln!("Unable to update a cat, no parameters");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOnePatchErrorNamed::NoParameters {
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
            eprintln!("please use post or maybe todo for full object update");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOnePatchErrorNamed::PleaseUsePost {
                please_use_post: std::string::String::from("please_use_post"),
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
            eprintln!("Unable to update a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresUpdateOnePatchErrorNamed::Update {
                update: e,
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

// curl -X DELETE http://127.0.0.1:8080/api/cats/delete_by_id/2
#[actix_web::delete("delete_by_id/{id}")]
pub async fn delete_by_id(
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdPathParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - check maybe not need to use everywhere InternalServerError
) -> impl actix_web::Responder {
    //todo how to handle sql injection ?
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
            eprintln!("Unable to update a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::DeleteByIdErrorNamed::Delete {
                delete: e,
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

// curl -X DELETE http://127.0.0.1:8080/api/cats/delete_where/?color=white
#[actix_web::delete("delete_where/")]
pub async fn delete_where(
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::DeleteWhereQueryParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - check maybe not need to use everywhere InternalServerError
) -> impl actix_web::Responder {
    //todo how to handle sql injection ?
    println!("delete_where name {:?}, color {:?}", query_parameters.name, query_parameters.color);
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            eprintln!("Unable to delete_where cats, no parameters");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresDeleteWhereErrorNamed::NoParameters {
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
            eprintln!("Unable to delete_where cats, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresDeleteWhereErrorNamed::Delete {
                delete: e,
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

//todo do not support put method - only for mongo