use serde_json::Value;

use crate::helpers::write_json_into_file::{write_json_into_file, WriteJsonIntoFileError};
use crate::providers::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn write_provider_json_into_file(
    json: Value,
    pk: ProviderKind,
    path: String,
) -> (ProviderKind, Result<(), WriteJsonIntoFileError>) {
    if let Err(e) = write_json_into_file(std::path::Path::new(&path), json) {
        return (pk, Err(e));
    }
    (pk, Ok(()))
}
