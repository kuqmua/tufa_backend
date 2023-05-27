// http://127.0.0.1:8080/api/cats/?limit=87 - Some(87)
//or
// http://127.0.0.1:8080/api/cats/ - None
#[actix_web::get("/")]
pub async fn get_all(
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::SelectAllQueryParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>, 
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> actix_web::HttpResponse {//or impl actix_web::Responder
    println!("get all cats, limit {:?}", query_parameters.limit);
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => &tufa_common::repositories_types::tufa_server::routes::cats::DEFAULT_SELECT_ALL_LIMIT
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "SELECT * FROM cats LIMIT $1",
        *limit as i64
    )
   .fetch_all(&**pool)
   .await {
        Ok(vec_cats) => {
            // tracing::info!("selected casts:\n{vec_cats:#?}");
            println!("selected casts:\n{vec_cats:#?}");
            actix_web::HttpResponse::Ok()
            .json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::GetAllResponse::Ok(vec_cats)
            ))
        },
        Err(e) => {
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresSelectAllCatsErrorNamed::SelectCats {
                select_cats: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            let error_with_serialize_deserialize = error.into_serialize_deserialize_version();
            actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    tufa_common::repositories_types::tufa_server::routes::cats::GetAllResponse::Err(error_with_serialize_deserialize)
                )
            )
        },
    }
}

#[actix_web::get("/{name}/{color}")]//#[actix_web::get("/{id}/{name}")]
pub async fn select_cats(
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::GetPathParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>, 
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - request metainfo
) -> actix_web::HttpResponse {//or impl actix_web::Responder
    //add check for 
    println!(
        "Welcome name {}, color {}!",
        // path_parameters.id, 
        path_parameters.name,
        path_parameters.color
    );
    println!("{}", {
        use tufa_common::common::config::config_fields::GetServerPort;
        config.get_server_port()
    });
//     //step1
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "SELECT * FROM cats WHERE name = $1",
        "black"
    )
   .fetch_all(&**pool)
   .await {
        Ok(vec_cats) => {
            //
            //step2
            match sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "INSERT INTO cats(name, color) VALUES ($1, $2) RETURNING *",
                "meowman", "white"
            )
            .fetch_all(&**pool)
            .await {
                Ok(vec_cats) => {
                    println!("{vec_cats:#?}");
                },
                Err(e) => {
                    eprintln!("Unable to insert a cat, error: {e:#?}");
                },
            }
            //step3
            match sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2 returning *",
                "black",
                1i64
            )
            .fetch_all(&**pool)
            .await {
                Ok(vec_cats) => {
                   println!("{vec_cats:#?}");
                },
                Err(e) => {
                    eprintln!("Unable to update a cat, error: {e:#?}");
                },
            }
            //
            // tracing::info!("selected casts:\n{vec_cats:#?}");
            println!("selected cats:\n{vec_cats:#?}");
            actix_web::HttpResponse::Ok()
            .json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::GetResponse::Ok(vec_cats)
            ))
        },
        Err(e) => {
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresSelectCatsErrorNamed::SelectCats {
                select_cats: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    tufa_common::repositories_types::tufa_server::routes::cats::GetResponse::Err(error.into_serialize_deserialize_version())
                )
            )
        },
    }
}

// curl -X POST http://127.0.0.1:8080/api/cats -H 'Content-Type: application/json' -d '[{"id":1,"name":"black"}]'
pub async fn post(
    vec_cat: actix_web::web::Json<Vec<tufa_common::repositories_types::tufa_server::routes::cats::Cat>>
) -> impl actix_web::Responder {
    println!("posted vec_cat\n{:#?}", vec_cat);
    actix_web::HttpResponse::Ok().finish()
}