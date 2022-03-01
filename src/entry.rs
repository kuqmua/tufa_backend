use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::get_providers_posts::get_providers_posts;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use std::time::Instant;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

extern crate num_cpus;

use crate::check_net::check_net_wrapper::check_net_wrapper;

use crate::init_dbs_logic::init_dbs::init_dbs;

use crate::helpers::get_git_source_file_link::get_git_source_file_link;

// use crate::check_net::check_net_enum::CheckNetError;

// use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;

// use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorEnum;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();
    print_colorful_message(
        None,
        PrintType::Info,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        format!("We are on a multicore system with {cpus} CPUs"),
    );
    // match tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(cpus)
    //     .enable_all()
    //     .build()
    // {
        
    // }
    // Err(e) => {
    //     print_colorful_message(
    //         None,
    //         PrintType::Error,
    //         vec![format!("{}:{}:{}", file!(), line!(), column!())],
    //         vec![get_git_source_file_link(file!(), line!())],
    //         format!("Cannot build tokio runtime {e:#?}"),
    //     );
    //     return;
    // }
    // Ok(runtime) => {
        
    // }
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
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        format!("preparation done in {} seconds", time.elapsed().as_secs()),
    );
    if CONFIG.is_dbs_initialization_enabled {
        if !CONFIG.is_mongo_initialization_enabled
            && !CONFIG.is_postgres_initialization_enabled
        {
            print_colorful_message(
                None,
                PrintType::WarningLow,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![get_git_source_file_link(file!(), line!())],
                String::from("db initialization for mongo and postgres are disabled"),
            );
        } else if let Err(e) = init_dbs().await {
            print_colorful_message(
                None,
                PrintType::Error,
                vec![],
                vec![],
                format!("{e:#?}"),
            );
        }
    }
    if ProviderKind::get_enabled_providers_vec().is_empty() {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![get_git_source_file_link(file!(), line!())],
            "all providers are disabled, get_providers_posts will not run".to_owned(),
        );
        return;
    };
    get_providers_posts().await;
    //move time measument in some inner part coz it would be server here
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        format!("main done in {} seconds", time.elapsed().as_secs()),
    );
}
