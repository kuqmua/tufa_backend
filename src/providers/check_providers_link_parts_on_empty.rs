use std::collections::HashMap;

use super::provider_kind_enum::ProviderKind;

#[derive(Debug)]
pub enum CheckEmptyError {
    Full,
    Partially(Vec<ProviderKind>),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn check_providers_link_parts_on_empty(
    providers_link_parts: HashMap<ProviderKind, Vec<String>>,
) -> Result<HashMap<ProviderKind, Vec<String>>, CheckEmptyError> {
    if providers_link_parts.is_empty() {
        return Err(CheckEmptyError::Full);
    }
    let mut non_empty_providers_link_parts = HashMap::with_capacity(providers_link_parts.len());
    let mut empty_providers_link_parts = HashMap::with_capacity(providers_link_parts.len());
    for (pk, vec) in providers_link_parts {
        if vec.is_empty() {
            empty_providers_link_parts.insert(pk, vec);
        } else {
            non_empty_providers_link_parts.insert(pk, vec);
        }
    }
    if !empty_providers_link_parts.is_empty() {
        let mut pk_vec = Vec::with_capacity(empty_providers_link_parts.len());
        for pk in empty_providers_link_parts.keys() {
            pk_vec.push(*pk);
        }
        return Err(CheckEmptyError::Partially(pk_vec));
    }
    Ok(non_empty_providers_link_parts)
}
