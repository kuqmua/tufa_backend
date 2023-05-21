// pub async fn json_example_post(json: actix_web::web::Json<Cats>) -> impl actix_web::Responder {
//     println!("json example {:#?}", json);
//     actix_web::HttpResponse::Ok().finish()
// }

pub async fn cats(pool: actix_web::web::Data<sqlx::PgPool>, config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>) -> actix_web::HttpResponse {//or impl actix_web::Responder
    println!("cats");
    println!("{}", {
        use tufa_common::common::config::config_fields::GetMongoUrl;
        use secrecy::ExposeSecret;
        config.get_mongo_url().expose_secret()
    });
    //step1
    let vec_cats_result = match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cats,
        "SELECT * FROM cats WHERE name = $1",
        "black"
    )
   .fetch_all(&**pool)
   .await {
        Ok(vec_cats) => {
            println!("{vec_cats:#?}");
            tufa_common::repositories_types::tufa_server::routes::cats::CatsResult::Ok(vec_cats)
        },
        Err(e) => {
            eprintln!("Unable to query cats table, error: {e:#?}");
            tufa_common::repositories_types::tufa_server::routes::cats::CatsResult::Err(std::string::String::from("Unable to query cats table, error: {e:#?}"))
        },
    };
    //step2
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::cats::Cats,
        "INSERT INTO cats(name) VALUES ($1) RETURNING *",
        "white"
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
        tufa_common::repositories_types::tufa_server::routes::cats::Cats,
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
    actix_web::HttpResponse::Ok()
    .json(actix_web::web::Json(vec_cats_result))
    // .finish()
}