pub async fn postgres_establish_connection(
    providers_json_local_data_hashmap: &std::collections::HashMap<crate::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<sqlx::Pool<sqlx::Postgres>, Box<tufa_common::server::postgres::postgres_establish_connection::PostgresEstablishConnectionErrorNamed>> {
    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(providers_json_local_data_hashmap.len() as u32)
        .connect_timeout(std::time::Duration::from_millis(crate::global_variables::runtime::config::CONFIG.postgres_connection_timeout)) //todo add timeout constant or env var
        .connect(&{
            use tufa_common::traits::get_postgres_url::GetPostgresUrl;
            crate::global_variables::runtime::config::CONFIG.get_postgres_url()
        })
        .await
    {
        Err(e) => Err(Box::new(
            tufa_common::server::postgres::postgres_establish_connection::PostgresEstablishConnectionErrorNamed::Connect { 
                sqlx_error: e, 
                code_occurence: tufa_common::code_occurence!(), 
            }
        )),
        Ok(pool) => Ok(pool),
    }
}
