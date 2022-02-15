pub trait PrintTypeTrait {
    fn is_error_prints_enabled(&self) -> bool;
    fn is_warning_high_prints_enabled(&self) -> bool;
    fn is_warning_low_prints_enabled(&self) -> bool;
    fn is_success_prints_enabled(&self) -> bool;
    fn is_partial_success_prints_enabled(&self) -> bool;
    fn is_time_measurement_prints_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool;
    fn is_info_prints_enabled(&self) -> bool;
}