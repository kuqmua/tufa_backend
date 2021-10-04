#[derive(Default, Debug, Clone, PartialEq)] //, serde_derive::Serialize, serde_derive::Deserialize
pub struct EnableMongoOwnUrlParts {
    pub mongo_own_first_handle_url_part: String,
    pub mongo_own_second_handle_url_part: String,
    pub mongo_own_third_handle_url_part: String,
    pub mongo_own_fourth_handle_url_part: String,
}
