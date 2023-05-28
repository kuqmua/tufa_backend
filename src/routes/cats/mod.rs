// http://127.0.0.1:8080/api/cats/?limit=87 - Some(87)
//or
// http://127.0.0.1:8080/api/cats/ - None
#[actix_web::get("/")]
pub async fn select(//todo rename to get(vec) and add adiitional parameters
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::cats::SelectQueryParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>, 
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - add check github commit id
) -> actix_web::HttpResponse {//or impl actix_web::Responder
    println!("select, limit {:?}, name {:?} color {:?}", 
        query_parameters.limit,
        query_parameters.name,
        query_parameters.color
    );
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => &tufa_common::repositories_types::tufa_server::routes::cats::DEFAULT_SELECT_ALL_LIMIT
    };
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        //match` arms have incompatible types if return just a result of query_as!
        (Some(name), Some(color)) => sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::cats::Cat,
            "SELECT * FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
            name,
            color,
            *limit as i64
        ).fetch_all(&**pool).await,
        (Some(name), None) => sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::cats::Cat,
            "SELECT * FROM cats WHERE name = $1 LIMIT $2",
            name,
            *limit as i64
        ).fetch_all(&**pool).await,
        (None, Some(color)) => sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::cats::Cat,
            "SELECT * FROM cats WHERE color = $1 LIMIT $2",
            color,
            *limit as i64
        ).fetch_all(&**pool).await,
        (None, None) => sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::cats::Cat,
            "SELECT * FROM cats LIMIT $1",
            *limit as i64
        ).fetch_all(&**pool).await,

    };
    match query_result {
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
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresSelectCatsErrorNamed::SelectCats {
                select_cats: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::GetAllResponse::Err(error.into_serialize_deserialize_version())
            ))
        },
    }
}
//http://127.0.0.1:8080/api/cats/756
#[actix_web::get("/{id}")]
pub async fn select_by_id(
    path_parameters: actix_web::web::Path<tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdPathParameters>,
    pool: actix_web::web::Data<sqlx::PgPool>, 
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> actix_web::HttpResponse {//or impl actix_web::Responder
    println!("select_by_id {}", path_parameters.id);
    let bigserial = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(path_parameters.id) {
        Ok(bigserial) => bigserial,
        Err(error) => {
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            return actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdResponse::BigserialError(error.into_serialize_deserialize_version())
                )
            );
        },
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial.bigserial()
    )
   .fetch_one(&**pool)
   .await {
        Ok(cat) => {
            // tracing::info!("selected casts:\n{vec_cats:#?}");
            println!("selected cats:\n{cat:#?}");
            actix_web::HttpResponse::Ok()
            .json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdResponse::Ok(cat)
            ))
        },
        Err(e) => {
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresSelectCatErrorNamed::SelectCat {
                select_cat: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    tufa_common::repositories_types::tufa_server::routes::cats::SelectByIdResponse::Select(error.into_serialize_deserialize_version())
                )
            )
        },
    }
}

// curl -X POST http://127.0.0.1:8080/api/cats/ -H 'Content-Type: application/json' -d '{"name":"simba", "color":"black"}'
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
    .await {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Unable to insert a cat, error: {e:#?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::PostgresInsertCatErrorNamed::InsertCat {
                select_cat: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            actix_web::HttpResponse::InternalServerError()
            .json(actix_web::web::Json(error.into_serialize_deserialize_version()))
        },
    }
}