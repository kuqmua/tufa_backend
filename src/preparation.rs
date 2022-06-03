use crate::check_net::check_net_wrapper::check_net_wrapper;
use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn preparation() {
    if let Err(e) = check_net_wrapper().await {
        println!("{e}");
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
        return;
    };
    if CONFIG.is_dbs_initialization_enabled {
        //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
        if !CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled {
            print_colorful_message(
                None,
                PrintType::WarningLow,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                String::from("db initialization for mongo and postgres are disabled"),
            );
        } else if let Err(e) = init_dbs().await {
            print_colorful_message(None, PrintType::Error, vec![], vec![], format!("{e:#?}"));
        }
    }
}
