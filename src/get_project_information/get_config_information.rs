use crate::overriding::prints::print_error_red;
use std::fs::File;
use std::io::prelude::*;
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
    pub params: Params,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,

    pub starting_check_link: String,
    pub arxiv_link: String,
    pub biorxiv_link: String,
    pub habr_link: String,
    pub medrxiv_link: String,
    pub reddit_link: String,
    pub twitter_link: String,

    pub enable_all_providers: bool,
    pub enable_arxiv: bool,
    pub enable_biorxiv: bool,
    pub enable_habr: bool,
    pub enable_medrxiv: bool,
    pub enable_reddit: bool,
    pub enable_twitter: bool,

    pub enable_all_providers_prints: bool,
    pub enable_prints_arxiv: bool,
    pub enable_prints_biorxiv: bool,
    pub enable_prints_habr: bool,
    pub enable_prints_medrxiv: bool,
    pub enable_prints_reddit: bool,
    pub enable_prints_twitter: bool,

    pub enable_warning_prints_for_all_providers: bool,
    pub enable_warning_prints_for_arxiv: bool,
    pub enable_warning_prints_for_biorxiv: bool,
    pub enable_warning_prints_for_habr: bool,
    pub enable_warning_prints_for_medrxiv: bool,
    pub enable_warning_prints_for_reddit: bool,
    pub enable_warning_prints_for_twitter: bool,

    pub enable_error_prints_for_all_providers: bool,
    pub enable_error_prints_for_arxiv: bool,
    pub enable_error_prints_for_biorxiv: bool,
    pub enable_error_prints_for_habr: bool,
    pub enable_error_prints_for_medrxiv: bool,
    pub enable_error_prints_for_reddit: bool,
    pub enable_error_prints_for_twitter: bool,

    pub enable_all_cleaning_warning_logs_directory: bool,
    pub enable_cleaning_warning_logs_directory_for_arxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_habr: bool,
    pub enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_reddit: bool,
    pub enable_cleaning_warning_logs_directory_for_twitter: bool,

    pub enable_prints_handle: bool,
    pub enable_error_prints_handle: bool,

    pub warning_logs_directory_name: String,

    pub enable_all_time_measurement: bool,
    pub enable_common_time_measurement: bool,
    pub enable_arxiv_time_measurement: bool,
    pub enable_biorxiv_time_measurement: bool,
    pub enable_habr_time_measurement: bool,
    pub enable_medrxiv_time_measurement: bool,
    pub enable_reddit_time_measurement: bool,
    pub enable_twitter_time_measurement: bool,
}

pub fn get_config_information() -> Option<Config> {
    let result_of_file_opening = File::open("././Config.toml");
    match result_of_file_opening {
        Ok(mut file) => {
            let mut string_file_content = String::new();
            let result_of_writing_to_string_from_file =
                file.read_to_string(&mut string_file_content);
            match result_of_writing_to_string_from_file {
                Ok(_) => {
                    let result_of_convertion_to_config_struct: std::result::Result<
                        Config,
                        toml::de::Error,
                    > = toml::from_str(&string_file_content);
                    match result_of_convertion_to_config_struct {
                        Ok(config) => Some(config),
                        Err(error) => {
                            let error_message =
                                format!("result_of_convertion_to_config_struct error {}", error);
                            print_error_red(
                                file!().to_string(),
                                line!().to_string(),
                                error_message,
                            );
                            None
                        }
                    }
                }
                Err(error) => {
                    let error_message =
                        format!("result_of_writing_to_string_from_file error {}", error);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                    None
                }
            }
        }
        Err(error) => {
            let error_message = format!("result_of_file_opening error {}", error);
            print_error_red(file!().to_string(), line!().to_string(), error_message);
            None
        }
    }
}
// let option_of_config = get_config_information();
//     match option_of_config {
//         Some(config) => {
//             println!("{:#?}config", config);
//         }
//         None => {
//             todo!()
//         }
//     }
