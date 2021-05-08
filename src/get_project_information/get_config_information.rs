use crate::overriding::prints::print_error_red;
use std::fs::File;
use std::io::prelude::*;
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
    params: Params,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    reddit_user_agent: String,
    reddit_client_id: String,
    reddit_client_secret: String,
    reddit_username: String,
    reddit_password: String,

    starting_check_link: String,
    arxiv_link: String,
    biorxiv_link: String,
    habr_link: String,
    medrxiv_link: String,
    reddit_link: String,
    twitter_link: String,

    enable_all_providers: bool,
    enable_arxiv: bool,
    enable_biorxiv: bool,
    enable_habr: bool,
    enable_medrxiv: bool,
    enable_reddit: bool,
    enable_twitter: bool,

    enable_all_providers_prints: bool,
    enable_prints_arxiv: bool,
    enable_prints_biorxiv: bool,
    enable_prints_habr: bool,
    enable_prints_medrxiv: bool,
    enable_prints_reddit: bool,
    enable_prints_twitter: bool,

    enable_warning_prints_for_all_providers: bool,
    enable_warning_prints_for_arxiv: bool,
    enable_warning_prints_for_biorxiv: bool,
    enable_warning_prints_for_habr: bool,
    enable_warning_prints_for_medrxiv: bool,
    enable_warning_prints_for_reddit: bool,
    enable_warning_prints_for_twitter: bool,

    enable_error_prints_for_all_providers: bool,
    enable_error_prints_for_arxiv: bool,
    enable_error_prints_for_biorxiv: bool,
    enable_error_prints_for_habr: bool,
    enable_error_prints_for_medrxiv: bool,
    enable_error_prints_for_reddit: bool,
    enable_error_prints_for_twitter: bool,

    enable_all_cleaning_warning_logs_directory: bool,
    enable_cleaning_warning_logs_directory_for_arxiv: bool,
    enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    enable_cleaning_warning_logs_directory_for_habr: bool,
    enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    enable_cleaning_warning_logs_directory_for_reddit: bool,
    enable_cleaning_warning_logs_directory_for_twitter: bool,

    enable_prints_handle: bool,
    enable_error_prints_handle: bool,

    warning_logs_directory_name: String,

    enable_all_time_measurement: bool,
    enable_common_time_measurement: bool,
    enable_arxiv_time_measurement: bool,
    enable_biorxiv_time_measurement: bool,
    enable_habr_time_measurement: bool,
    enable_medrxiv_time_measurement: bool,
    enable_reddit_time_measurement: bool,
    enable_twitter_time_measurement: bool,
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
