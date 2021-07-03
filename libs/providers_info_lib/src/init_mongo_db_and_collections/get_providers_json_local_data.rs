use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn get_providers_json_local_data(
    path: &str,
    vec_of_provider_names: std::vec::Vec<&'static str>,
    second_part_of_file_name: &str,
    file_extension: &str,
) -> HashMap<&'static str, Vec<String>> {
    let mut vec_of_link_parts_hashmap: HashMap<&str, Vec<String>> = HashMap::new();
    //todo: do it async in parallel
    for provider_name in vec_of_provider_names {
        let result_of_opening_file = File::open(format!(
            "{}{}{}{}",
            path, provider_name, second_part_of_file_name, file_extension
        ));
        match result_of_opening_file {
            Ok(mut file) => {
                let mut contents = String::new();
                let result_of_reading_to_string = file.read_to_string(&mut contents);
                match result_of_reading_to_string {
                    Ok(_) => {
                        let file_content_from_str_result: Result<
                            ProviderLinkPartStruct,
                            serde_json::Error,
                        > = serde_json::from_str(&contents);
                        match file_content_from_str_result {
                            Ok(file_content_as_struct) => {
                                let mut vec_of_link_parts: Vec<String> =
                                    Vec::with_capacity(file_content_as_struct.data.len());
                                // println!(
                                //     "file_content_as_struct {:#?}",
                                //     file_content_as_struct.data
                                // );
                                for link_part in file_content_as_struct.data {
                                    vec_of_link_parts.push(link_part)
                                }
                                vec_of_link_parts_hashmap.insert(provider_name, vec_of_link_parts);
                            }
                            Err(e) => panic!("file_content_from_str_result error {:#?}", e),
                        }
                    }
                    Err(e) => {
                        panic!(
                            "cannot read_to_string from file {}{}{}{}, reason: {}",
                            path, provider_name, second_part_of_file_name, file_extension, e
                        )
                    }
                }
            }
            Err(e) => {
                panic!(
                    "cannot open {}{}{}{}, reason: {}",
                    path, provider_name, second_part_of_file_name, file_extension, e
                )
            }
        }
    }
    // println!("vec_of_link_parts_hashmap {:#?}", vec_of_link_parts_hashmap)
    vec_of_link_parts_hashmap
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ProviderLinkPartStruct {
    pub data: Vec<String>,
}
