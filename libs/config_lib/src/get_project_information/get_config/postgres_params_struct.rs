#[derive(Default, Debug, Clone, PartialEq)]
pub struct PostgresParams {
    pub postgres_is_cloud: String,
    pub postgres_own_first_handle_url_part: String,
    pub postgres_own_second_handle_url_part: String,
    pub postgres_own_third_handle_url_part: String,
    pub postgres_own_fourth_handle_url_part: String,
}
