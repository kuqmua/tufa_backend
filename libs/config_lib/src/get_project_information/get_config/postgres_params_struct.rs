#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PostgresParams {
    pub postgres_own_first_handle_url_part: String,
    pub postgres_own_second_handle_url_part: String,
    pub postgres_own_third_handle_url_part: String,
    pub postgres_own_fourth_handle_url_part: String,
}
