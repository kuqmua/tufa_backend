#[derive(Default, Debug, Clone, PartialEq)]
pub struct MongoAuthorization {
    pub mongo_login: String,
    pub mongo_password: String,
    pub mongo_ip: String,
    pub mongo_port: String,
    pub mongo_params: String,
}
