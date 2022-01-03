//do not comment any code in this file coz it will be parsed iside proc_macro
pub trait ProviderKindFromConfigTrait {
    fn is_mongo_initialization_enabled(&self) -> bool;
    fn is_mongo_write_error_logs_enabled(&self) -> bool;
    fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool;
    fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool;
    fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool;

    fn is_postgres_initialization_enabled(&self) -> bool;

    fn is_write_error_logs_in_local_folder_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool;

    fn check_link(&self) -> String;

    fn is_enabled(&self) -> bool;
    fn is_prints_enabled(&self) -> bool;
    fn is_warning_high_prints_enabled(&self) -> bool;
    fn is_warning_low_prints_enabled(&self) -> bool;
    fn is_success_prints_enabled(&self) -> bool;
    fn is_partial_success_prints_enabled(&self) -> bool;
    fn is_error_prints_enabled(&self) -> bool;
    fn is_time_measurement_prints_enabled(&self) -> bool;
    fn is_info_prints_enabled(&self) -> bool;

    fn is_links_limit_enabled(&self) -> bool;
    fn links_limit(&self) -> i64;
}
