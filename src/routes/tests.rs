#[derive(Debug)]
struct Domain {
  id: i64,
  name: String
}

pub async fn tests(pool: actix_web::web::Data<sqlx::PgPool>) -> actix_web::HttpResponse {
    println!("tests");
    let domains: Vec<Domain> = sqlx::query_as!(
        Domain,
        "select * from domains where name = $1",
        "my-2.domain.com"
    )
   .fetch_all(&**pool)
   .await.expect("Unable to query domains table");
    println!("{domains:#?}");

// let inserted_domains: Vec<Domain> = sqlx::query_as!(
//   Domain,
//   "insert into domains(name) values ($1) returning *",
//   "my.domain.com"
// )
//   .fetch_all(&pool)
//   .await.expect("Unable to insert a domain");

// println!("{inserted_domains:#?}");


// let updated_domains = sqlx::query_as!(
//   Domain,
//   "update domains set name = 'my-2.domain.com' where id = $1 returning *",
//   1i64
// )
//   .fetch_all(&pool)
//   .await.expect("Unable to update a domain");

// println!("{updated_domains:#?}");
    //
    actix_web::HttpResponse::Ok().finish()
}