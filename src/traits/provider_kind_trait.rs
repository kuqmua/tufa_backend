use mongodb::bson::Document;

pub trait ProviderKindTrait {
    fn is_link_limits_enabled(&self) -> bool;
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool;
    fn get_check_link(&self) -> &'static str;
    fn get_init_local_data_file_path(&self) -> String;
    fn get_item_handle(&self) -> Option<&'static str>;
    fn get_links_limit_for_provider(&self) -> i64;
    fn get_mongo_doc_randomization_aggregation(&self) -> Option<Document>;
    fn get_mongo_log_collection_name(&self) -> String;
    fn get_path_to_logs_directory(&self) -> String;
    fn get_path_to_provider_log_file(&self) -> String;
    fn get_provider_links(&self, names_vector: Vec<String>) -> Vec<String>;
}
