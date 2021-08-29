use crate::fetch::rss_write_error_logs_into_file_for_provider_wrapper_checker::rss_write_error_logs_into_file_for_provider_wrapper_checker;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use serde_json::Value;
use std::time::Instant;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn async_write_json_into_file(
    json_object: Value,
    provider_kind: ProviderKind,
    unhandled_success_handled_success_are_there_items_initialized_posts_dir: &str,
    warning_logs_directory_name: &str,
    link: String,
) {
    let time = Instant::now();
    rss_write_error_logs_into_file_for_provider_wrapper_checker(
        json_object,
        &provider_kind,
        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
        warning_logs_directory_name,
        &link,
    );
    print_colorful_message(
        Some(&provider_kind),
        PrintType::TimeMeasurement,
        file!().to_string(),
        line!().to_string(),
        format!(
            "write fetch error logs into file done in {} seconds {} miliseconds for {:#?}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
            provider_kind
        ),
    );
}
