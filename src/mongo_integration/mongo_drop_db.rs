use mongodb::{options::ClientOptions, Client};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_db(mongo_url: &str, db_name: &str) -> Result<bool, mongodb::error::Error> {
    let result_flag: bool;
    let client_options_result = ClientOptions::parse(mongo_url).await;
    match client_options_result {
        Ok(client_options) => {
            let client_result = Client::with_options(client_options);
            match client_result {
                Ok(client) => {
                    let db = client.database(db_name);
                    let drop_db_result = db.drop(None).await;
                    match drop_db_result {
                        Ok(_) => result_flag = true,
                        Err(e) => {
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                file!().to_string(),
                                line!().to_string(),
                                format!("Drop failed {:#?}", e),
                            );
                            result_flag = false;
                        }
                    }
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        file!().to_string(),
                        line!().to_string(),
                        format!("Client::with_options error {:#?}", e),
                    );
                    result_flag = false;
                }
            }
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("ClientOptions::parse error {:#?}", e),
            );
            result_flag = false;
        }
    }
    Ok(result_flag)
}
