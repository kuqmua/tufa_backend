#[derive(Default, Debug, Clone, PartialEq)]
pub struct Params {
    pub vec_of_provider_names: Vec<String>,
    pub starting_check_link: String,
    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub enable_cleaning_warning_logs_db_in_mongo: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo: bool,
    pub enable_time_measurement: bool,
    pub enable_provider_links_limit: bool,
    pub enable_common_providers_links_limit: bool,
    pub common_providers_links_limit: i64,
    pub enable_randomize_order_for_providers_link_parts_for_mongo: bool,
    //
    pub enable_prints: bool,
    pub enable_error_prints: bool,
    pub enable_warning_high_prints: bool,
    pub enable_warning_low_prints: bool,
    pub enable_success_prints: bool,
    pub enable_partial_success_prints: bool,
    pub enable_time_measurement_prints: bool,
    pub enable_cleaning_warning_logs_directory_prints: bool,
    pub enable_info_prints: bool,
    //
    pub enable_write_error_logs_in_local_folder: bool,
    pub enable_write_error_logs_in_mongo: bool,
}
