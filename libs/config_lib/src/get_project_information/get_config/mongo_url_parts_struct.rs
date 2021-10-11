#[derive(Default, Debug, Clone, PartialEq)] //, serde_derive::Serialize, serde_derive::Deserialize
pub struct MongoUrlParts {
    pub mongo_first_handle_url_part: String,
    pub mongo_second_handle_url_part: String,
    pub mongo_third_handle_url_part: String,
    pub mongo_fourth_handle_url_part: String,
    pub mongo_fifth_handle_url_part: String,
}
