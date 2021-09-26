#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PostgresOwnAuthorization {
    pub postgres_own_login: String,
    pub postgres_own_password: String,
    pub postgres_own_ip: String,
    pub postgres_own_db: String,
}
