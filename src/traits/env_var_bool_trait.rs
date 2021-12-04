pub trait EnvVarBoolTrait {
    fn is_mongo_initialization_enabled(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn is_prints_enabled(&self) -> bool;
    fn is_warning_high_prints_enabled(&self) -> bool;
    fn is_warning_low_prints_enabled(&self) -> bool;
    fn is_error_prints_enabled(&self) -> bool;
    fn is_success_prints_enabled(&self) -> bool;
    fn is_partial_success_prints_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_db_in_mongo_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_db_collections_in_mongo_enabled(&self) -> bool;
    fn is_time_measurement_enabled(&self) -> bool;
    fn is_info_prints_enabled(&self) -> bool;
    fn is_link_limits_enabled(&self) -> bool;
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enable(&self) -> bool;
}
