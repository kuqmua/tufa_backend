use std::fmt;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_availability::CheckNetAvailabilityError;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(thiserror::Error, displaydoc::Display, Debug, ImplDisplayDerive)]
pub struct CheckNetWrapperError {
    /// check net wrapper error {source:?}
    #[source]
    pub source: Box<CheckNetWrapperErrorEnum>,
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, displaydoc::Display, Debug, ImplFromForUpperStruct)]
pub enum CheckNetWrapperErrorEnum {
    /// net check availability error {0:?}
    CheckNetAvailabilityError(CheckNetAvailabilityError),
    /// postgres check availability error {0:?}
    PostgresCheckAvailabilityError(PostgresCheckAvailabilityError),
    /// mongo check availability error {0:?}
    MongoCheckAvailabilityError(MongoCheckAvailabilityError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), CheckNetWrapperError> {
    //todo to it in parallel? no point for this yet coz dont know what db to use
    check_net_availability(&CONFIG.starting_check_link)?;
    print_colorful_message(
        None,
        PrintType::Info,
        file!().to_string(),
        line!().to_string(),
        "starting check postgres availability... ".to_owned(),
    );
    postgres_check_availability(&postgres_get_db_url())?;
    print_colorful_message(
        None,
        PrintType::Info,
        file!().to_string(),
        line!().to_string(),
        "starting check mongo availability... ".to_owned(),
    );
    mongo_check_availability(&mongo_get_db_url())?;
    Ok(())
}
