#[derive(serde::Deserialize)]
pub struct CatFilter {
    // id: i64,
    name: String,
    color: String,
}

#[actix_web::get("?name={name}/color={color}")]//#[actix_web::get("/{id}/{name}")]
pub async fn select_cats(
    route_path_info: actix_web::web::Path<CatFilter>,
    pool: actix_web::web::Data<sqlx::PgPool>, 
    config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>,
    //todo - request metainfo
) -> actix_web::HttpResponse {//or impl actix_web::Responder
    //add check for 
    println!(
        "Welcome id: {}, name {}, color {}!",
        route_path_info.id, route_path_info.name, route_path_info.color
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
            let error_wrapper = tufa_common::repositories_types::tufa_server::routes::cats::get::PostgresSelectCatsError {
                error: error_with_serialize_deserialize,
                port: {
                    use tufa_common::common::config::config_fields::GetServerPort;
                    config.get_server_port().clone()
                },
                pid: std::process::id(),
            };
            println!("{error_wrapper}");
            actix_web::HttpResponse::InternalServerError()
            .json(
                actix_web::web::Json(
                    tufa_common::repositories_types::tufa_server::routes::cats::get::GetResponse::Err(error_wrapper)
                )
            )
        },
    }
}