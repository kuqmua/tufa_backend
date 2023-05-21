#[derive(Debug)]
struct Domain {
  id: i64,
  name: String
}

pub async fn tests(pool: actix_web::web::Data<sqlx::PgPool>, config: actix_web::web::Data<&tufa_common::repositories_types::tufa_server::config::config_struct::Config>) -> actix_web::HttpResponse {
    println!("tests");
    println!("{}", {
        use tufa_common::common::config::config_fields::GetMongoUrl;
        use secrecy::ExposeSecret;
        config.get_mongo_url().expose_secret()
    });
    //step1
    match sqlx::query_as!(
        Domain,
        "SELECT * FROM domains WHERE name = $1",
        "my-2.domain.com"
    )
   .fetch_all(&**pool)
   .await {
        Ok(vec_domain) => {
            println!("{vec_domain:#?}");
        },
        Err(e) => {
            eprintln!("Unable to query domains table, error: {e:#?}");
        },
    }
    //step2
    match sqlx::query_as!(
        Domain,
        "INSERT INTO domains(name) VALUES ($1) RETURNING *",
        "my.domain.com"
    )
    .fetch_all(&**pool)
    .await {
        Ok(vec_domain) => {
            println!("{vec_domain:#?}");
        },
        Err(e) => {
            eprintln!("Unable to insert a domain, error: {e:#?}");
        },
    }
    //step3
    match sqlx::query_as!(
        Domain,
        "UPDATE domains SET name = $1 WHERE id = $2 returning *",
        "my-2.domain.com",
        1i64
    )
    .fetch_all(&**pool)
    .await {
        Ok(vec_domain) => {
                println!("{vec_domain:#?}");
            },
        Err(e) => {
            eprintln!("Unable to update a domain, error: {e:#?}");
        },
    }
    actix_web::HttpResponse::Ok().finish()
}