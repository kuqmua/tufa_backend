use serde_json::Value;

use crate::helpers::write_json_into_file::{write_json_into_file, WriteJsonIntoFileError};
use crate::providers::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn rss_write_error_logs_into_file_for_provider_wrapper_checker(
    json: Value,
    provider_kind: ProviderKind,
    path: String,
) -> (ProviderKind, Result<(), WriteJsonIntoFileError>) {
    if let Err(e) = write_json_into_file(std::path::Path::new(&path), json) {
        return (provider_kind, Err(e));
    }
    (provider_kind, Ok(()))
}
