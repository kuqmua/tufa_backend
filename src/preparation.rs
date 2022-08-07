use crate::check_net::check_net_wrapper::{check_net_wrapper, CheckNetWrapperError};
use crate::config_mods::lazy_static_config::CONFIG;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_tables_enum::InitTablesEnumError;
use std::fmt::Display;

#[derive(Debug)]
pub enum PreparationErrorEnum {
    CheckNet {
        source: Box<CheckNetWrapperError>,
    },
    InitDbs {
        source: Vec<Box<InitTablesEnumError>>,
    },
}

impl Display for PreparationErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                PreparationErrorEnum::CheckNet { source } => write!(f, "{}", *source),
                PreparationErrorEnum::InitDbs { source } => write!(f, "{:#?}", source), //todo
            },
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn preparation() -> Result<(), Box<PreparationErrorEnum>> {
    if let Err(e) = check_net_wrapper().await {
        // let sources = e
        //     .source
        //     .iter()
        //     .map(|e| match e {
        //         CheckNetError::Net(e) => match e {
        //             CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //             CheckNetAvailabilityErrorEnum::StatusCodeError {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //         },
        //         CheckNetError::Postgres(e) => e.where_was.source_place_with_readable_time(),
        //         CheckNetError::Mongo(e) => match e {
        //             MongoCheckAvailabilityErrorEnum::ClientOptionsParse {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //             MongoCheckAvailabilityErrorEnum::ClientWithOptions {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //             MongoCheckAvailabilityErrorEnum::ListCollectionNames {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //         },
        //     })
        //     .collect::<Vec<String>>();
        // let github_sources = e
        //     .source
        //     .iter()
        //     .map(|e| match e {
        //         CheckNetError::Net(e) => match e {
        //             CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //             CheckNetAvailabilityErrorEnum::StatusCodeError {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //         },
        //         CheckNetError::Postgres(e) => e.where_was.source_place_with_readable_time(),
        //         CheckNetError::Mongo(e) => match e {
        //             MongoCheckAvailabilityErrorEnum::ClientOptionsParse {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //             MongoCheckAvailabilityErrorEnum::ClientWithOptions {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //             MongoCheckAvailabilityErrorEnum::ListCollectionNames {
        //                 source: _,
        //                 where_was,
        //             } => where_was.source_place_with_readable_time(),
        //         },
        //     })
        //     .collect::<Vec<String>>();
        // print_colorful_message(
        //     None,
        //     PrintType::WarningHigh,
        //     sources,
        //     github_sources,
        //     format!("{e:#?}"),
        // );
        return Err(Box::new(PreparationErrorEnum::CheckNet { source: e }));
    };
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    if let Err(e) = init_dbs().await {
        return Err(Box::new(PreparationErrorEnum::InitDbs { source: e }));
    }
    Ok(())
}
