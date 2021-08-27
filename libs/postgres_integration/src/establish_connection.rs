use diesel::pg::PgConnection;
use diesel::prelude::*;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

pub fn establish_connection(database_url: String) -> Option<PgConnection> {
    let result_establish_connection = PgConnection::establish(&database_url);
    match result_establish_connection {
        Ok(pg_connection) => Some(pg_connection),
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("PgConnection::establish {} error: {:#?}", &database_url, e),
            );
            None
        }
    }
}
