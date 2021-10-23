use crate::config::get_config::postgres_authorization_struct::PostgresAuthorization;
use crate::config::get_config::postgres_url_parts_struct::PostgresUrlParts;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PostgresParams {
    pub postgres_url_parts: PostgresUrlParts,
    pub postgres_authorization: PostgresAuthorization,
}
