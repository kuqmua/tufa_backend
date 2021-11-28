use crate::providers::provider_kind_enum::ProviderKind;

use std::collections::HashMap;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(Debug)]
pub enum GetProvidersJsonLocalDataProcessedError {
    SerdeJsonErrors(Vec<serde_json::Error>),
    StdIoError(std::io::Error),
}

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_providers_json_local_data_processed() -> (
        HashMap<ProviderKind, Vec<String>>,
        HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>,
    ) {
        let unprocessed_hashmap = ProviderKind::get_providers_json_local_data_unprocessed();
        let mut first_return_handle: HashMap<ProviderKind, Vec<String>> =
            HashMap::with_capacity(unprocessed_hashmap.len());
        let mut second_return_handle: HashMap<
            ProviderKind,
            GetProvidersJsonLocalDataProcessedError,
        > = HashMap::with_capacity(unprocessed_hashmap.len());
        for (provider_kind, result) in unprocessed_hashmap {
            match result {
                Ok(second_result) => {
                    let mut serde_json_error_vec = Vec::<serde_json::Error>::new();
                    match second_result {
                        Ok(vec) => {
                            first_return_handle.insert(provider_kind, vec);
                        }
                        Err(e) => {
                            print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::get_providers_json_local_data_unprocessed ({:#?}), error: {:#?}", provider_kind, e),
                                );
                            serde_json_error_vec.push(e);
                        }
                    }
                    if !serde_json_error_vec.is_empty() {
                        second_return_handle.insert(
                            provider_kind,
                            GetProvidersJsonLocalDataProcessedError::SerdeJsonErrors(
                                serde_json_error_vec,
                            ),
                        );
                    }
                }
                Err(e) => {
                    print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::get_providers_json_local_data_unprocessed ({:#?}), error: {:#?}", provider_kind, e),
                                );
                    second_return_handle.insert(
                        provider_kind,
                        GetProvidersJsonLocalDataProcessedError::StdIoError(e),
                    );
                }
            }
        }
        (first_return_handle, second_return_handle)
    }
}
