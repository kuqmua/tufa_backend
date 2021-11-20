use serde_json::Value;

use crate::helpers::create_dir_if_it_doesnt_exist::create_dir_if_it_doesnt_exist;
use crate::helpers::write_json_into_file::write_json_into_file;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::fs::File;
use std::io::ErrorKind;

use std::path::Path;
use std::time::Instant;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn rss_write_error_logs_into_file_for_provider_wrapper_checker(
    json: Value,
    provider_kind: ProviderKind,
    path: String,
) -> Result<(), std::io::Error> {
    let result = write_json_into_file(std::path::Path::new(&path), json);
    match result {
        Ok(_) => println!("1"),
        Err(e) => println!("2 {:#?}", e),
    }
    Ok(())
}
