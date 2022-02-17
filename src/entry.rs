use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::get_providers_posts::get_providers_posts;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use crate::traits::where_was_trait::WhereWasTrait;
use std::time::Instant;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

extern crate num_cpus;

use crate::check_net::check_net_wrapper::check_net_wrapper;

use crate::init_dbs_logic::init_dbs::{init_dbs, InitDbsErrorEnum};

use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsErrorEnum;

use crate::helpers::get_git_source_file_link::get_git_source_file_link;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();
    print_colorful_message(
        None,
        PrintType::Info,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        format!("We are on a multicore system with {cpus} CPUs"),
    );
    if cpus == 0 {
        print_colorful_message(
            None,
            PrintType::Error,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![get_git_source_file_link(file!(), line!())],
            format!("CPU number == {cpus}, aborting"),
        );
        return;
    }
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cpus)
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
            return;
        }
        Ok(runtime) => {
            if let Err(e) = runtime.block_on(check_net_wrapper()) {
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![get_git_source_file_link(file!(), line!())],
                    format!("{e:#?}"),
                );
                return;
            }
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
                } else if let Err(e) = runtime.block_on(init_dbs()) {
                    match *e.source {
                    //its only one variant for now. later going to add more
                    InitDbsErrorEnum::InitDbsProvidersLinkParts { source, where_was: where_was1 } => match *source.source {
                        InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::MongoClientOptionsParse { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::MongoClientWithOptions { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::MongoCollectionCountDocumentsOrIsNotEmpty { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::MongoInsertManyError { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::PostgresDeleteAllFromProvidersTables { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::PostgresCheckProvidersLinkPartsTablesEmptyError { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::PostgresCreateTableQueries { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::PostgresInsertLinkPartsIntoProvidersTables { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                        InitDbsProvidersLinkPartsErrorEnum::PostgresEstablishConnection { source, where_was: where_was0 } => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                vec![
                                    where_was0.source_place(),
                                    where_was1.source_place(),
                                ],
                                vec![
                                    where_was0.github_source_place(),
                                    where_was1.github_source_place(),
                                ],
                                format!("{source:#?}"),
                            );
                        },
                    },
                }
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
            runtime.block_on(get_providers_posts());
        }
    }
    //move time measument in some inner part coz it would be server here
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        format!("main done in {} seconds", time.elapsed().as_secs()),
    );
}
