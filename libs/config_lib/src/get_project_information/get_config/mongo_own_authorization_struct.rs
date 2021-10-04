#[derive(Default, Debug, Clone, PartialEq)]
pub struct MongoOwnAuthorization {
    pub mongo_own_login: String,
    pub mongo_own_password: String,
    pub mongo_own_ip: String,
    pub mongo_own_port: String,
}
