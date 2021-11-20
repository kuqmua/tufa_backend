use std::collections::HashMap;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers::providers_info::get_providers_link_parts::get_providers_link_parts_as_hashmap;

use crate::helpers::resource::Resource;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_wrapper() -> Option<HashMap<ProviderKind, Vec<String>>> {
    //todo: write Resource logic
    let providers_link_parts = get_providers_link_parts_as_hashmap(&Resource::Mongodb {}).await;
    if !providers_link_parts.is_empty() {
        Some(providers_link_parts)
    } else {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "providers_link_parts .is_empty".to_string(),
        );
        let providers_link_parts_local =
            get_providers_link_parts_as_hashmap(&Resource::Local).await;
        if !providers_link_parts_local.is_empty() {
            Some(providers_link_parts_local)
        } else {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "providers_link_parts_local.is_empty too".to_string(),
            );
            None
        }
    }
}
