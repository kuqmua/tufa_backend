use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> HashMap<ProviderKind, Vec<String>> {
    //todo: return different type as errors or success enum
    match resource {
        Resource::Local => ProviderKind::get_providers_json_local_data(),
        Resource::Mongodb => {
            let mongo_result = ProviderKind::mongo_get_providers_link_parts(
                ProviderKind::get_enabled_providers_vec(),
            )
            .await;
            match mongo_result {
                Ok(hashmap_with_possible_errors) => {
                    let mut hashmap_without_possible_errors =
                        HashMap::with_capacity(hashmap_with_possible_errors.len());
                    for (provider_kind, result_vec) in hashmap_with_possible_errors {
                        match result_vec {
                            Ok(vec) => {
                                hashmap_without_possible_errors.insert(provider_kind, vec);
                            }
                            Err(e) => {
                                print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::mongo_get_providers_link_parts ({:#?}), error: {:#?}", provider_kind, e),
                                );
                                //todo: just create Vec::new() -> bad solution
                                hashmap_without_possible_errors
                                    .insert(provider_kind, Vec::<String>::new());
                            }
                        }
                    }
                    hashmap_without_possible_errors
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "(todo!)ProviderKind::mongo_get_providers_link_parts error: {:#?}",
                            e
                        ),
                    );
                    //todo: just create Vec::new() -> bad solution
                    ProviderKind::generate_hashmap_with_empty_string_vecs_for_enabled_providers()
                }
            }
        }
        Resource::PostgreSql => {
            todo!()
        }
    }
}
