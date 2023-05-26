pub async fn get(
    pool: actix_web::web::Data<sqlx::PgPool>, 
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
) -> actix_web::HttpResponse {//or impl actix_web::Responder
    // println!("{}", {
    //     use tufa_common::common::config::config_fields::GetMongoUrl;
    //     use secrecy::ExposeSecret;
    //     config.get_mongo_url().expose_secret()
    // });
//     //step1
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cat,
        "SELECT * FROM cats WHERE name = $1",
        "black"
    )
   .fetch_all(&**pool)
   .await {
        Ok(vec_cats) => {
            // tracing::info!("selected casts:\n{vec_cats:#?}");
            println!("selected casts:\n{vec_cats:#?}");
            actix_web::HttpResponse::Ok()
            .json(actix_web::web::Json(
                tufa_common::repositories_types::tufa_server::routes::cats::get::GetResponse::Ok(vec_cats)
            ))
        },
        Err(e) => {
            //todo port and pid ?
            // tracing::error!("Unable to query cats table, error: {e:?}");
            let error = tufa_common::repositories_types::tufa_server::routes::cats::get::PostgresSelectCatsErrorNamed::SelectCats {
                select_cats: e,
                code_occurence: tufa_common::code_occurence!(),
            };
            use tufa_common::common::error_logs_logic::error_log::ErrorLog;
            error.error_log(**config);
            let error_with_serialize_deserialize = error.into_serialize_deserialize_version();
            actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    tufa_common::repositories_types::tufa_server::routes::cats::get::GetResponse::Err(error_with_serialize_deserialize)
                )
            )
        },
    }
//     //step2
//     match sqlx::query_as!(
//         tufa_common::repositories_types::tufa_server::routes::cats::Cat,
//         "INSERT INTO cats(name) VALUES ($1) RETURNING *",
//         "white"
//     )
//     .fetch_all(&**pool)
//     .await {
//         Ok(vec_cats) => {
//             println!("{vec_cats:#?}");
//         },
//         Err(e) => {
//             eprintln!("Unable to insert a cat, error: {e:#?}");
//         },
//     }
//     //step3
//     match sqlx::query_as!(
//         tufa_common::repositories_types::tufa_server::routes::cats::Cat,
//         "UPDATE cats SET name = $1 WHERE id = $2 returning *",
//         "black",
//         1i64
//     )
//     .fetch_all(&**pool)
//     .await {
//         Ok(vec_cats) => {
//                 println!("{vec_cats:#?}");
//             },
//         Err(e) => {
//             eprintln!("Unable to update a cat, error: {e:#?}");
//         },
//     }
}