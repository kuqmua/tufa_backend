use std::collections::HashMap;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;

use std::fs;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_providers_json_local_data_unprocessed(
    ) -> HashMap<ProviderKind, Result<Result<Vec<String>, serde_json::Error>, std::io::Error>> {
        let mut vec_of_link_parts_hashmap: HashMap<
            ProviderKind,
            Result<Result<Vec<String>, serde_json::Error>, std::io::Error>,
        > = HashMap::with_capacity(ProviderKind::get_enabled_providers_vec().len());
        //todo: do it async in parallel
        for provider_kind in ProviderKind::get_enabled_providers_vec() {
            let result_of_reading_to_string =
                fs::read_to_string(&ProviderKind::get_init_local_data_file_path(provider_kind));
            match result_of_reading_to_string {
                Ok(file_content) => {
                    let file_content_from_str_result: Result<
                        ProvidersInitJsonSchema,
                        serde_json::Error,
                    > = serde_json::from_str(&file_content);
                    match file_content_from_str_result {
                        Ok(file_content_as_struct) => {
                            let mut vec_of_link_parts: Vec<String> =
                                Vec::with_capacity(file_content_as_struct.data.len());
                            for link_part in file_content_as_struct.data {
                                vec_of_link_parts.push(link_part)
                            }
                            vec_of_link_parts_hashmap
                                .insert(provider_kind, Ok(Ok(vec_of_link_parts)));
                        }
                        Err(e) => {
                            vec_of_link_parts_hashmap.insert(provider_kind, Ok(Err(e)));
                        }
                    }
                }
                Err(e) => {
                    vec_of_link_parts_hashmap.insert(provider_kind, Err(e));
                }
            }
        }
        vec_of_link_parts_hashmap
    }
}
