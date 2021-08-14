use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::{fs::File, io::Write};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_error_logs_into_file_for_provider(
    file_name: String,
    stringified_json: String,
) -> bool {
    let result_of_creating_file = File::create(&file_name);
    match result_of_creating_file {
        Ok(mut created_for_logs_file) => {
            let result_of_writing = created_for_logs_file.write(stringified_json.as_bytes()); //warning_message
            match result_of_writing {
                Ok(_) => {
                    print_colorful_message(
                        None,
                        PrintType::Success,
                        file!().to_string(),
                        line!().to_string(),
                        format!("logs were written in file {}", &file_name),
                    );
                    true
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!("error writing in file {} {}", file_name, e.to_string()),
                    );
                    false
                }
            }
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("error creating file {} {}", &file_name, e.to_string()),
            );
            false
        }
    }
}
