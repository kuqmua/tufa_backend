#[derive(Default, Debug, Clone, PartialEq)]
pub struct PostgresAuthorization {
    pub postgres_login: String,
    pub postgres_password: String,
    pub postgres_ip: String,
    pub postgres_port: String,
    pub postgres_db: String,
}
