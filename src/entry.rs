use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::get_providers_posts::get_providers_posts;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use std::time::Instant;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

extern crate num_cpus;

use crate::check_net::check_net_wrapper::check_net_wrapper;

use crate::init_dbs_logic::init_dbs::{init_dbs, InitDbsError};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();
    print_colorful_message(
        None,
        PrintType::Info,
        file!().to_string(),
        line!().to_string(),
        format!("We are on a multicore system with {} CPUs", cpus),
    );
    if cpus == 0 {
        print_colorful_message(
            None,
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!("CPU number == {}, aborting", cpus),
        );
        return;
    }
    if let Err(e) = check_net_wrapper() {
        print_colorful_message(
            None,
            PrintType::WarningHigh,
            file!().to_string(),
            line!().to_string(),
            format!("check_net_wrapper error: {:#?}", e),
        );
        return;
    }
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        file!().to_string(),
        line!().to_string(),
        format!(
            "preparation done in {} seconds, starting tokio runtime...",
            time.elapsed().as_secs()
        ),
    );
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cpus)
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("Cannot build tokio runtime {:#?}", e),
            );
            return;
        }
        Ok(runtime) => {
            if CONFIG.dbs_enable_initialization {
                if !CONFIG.mongo_enable_initialization && !CONFIG.postgres_enable_initialization {
                    print_colorful_message(
                        None,
                        PrintType::WarningLow,
                        file!().to_string(),
                        line!().to_string(),
                        String::from("db initialization for mongo and postgres are disabled"),
                    );
                    return;
                }
                if let Err(e) = runtime.block_on(init_dbs()) {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!("init dbs error {:#?}", e),
                    );
                    match e {
                        InitDbsError::GetProvidersJsonLocalData(_) => return,
                        InitDbsError::MongoClientOptionsParse(_) => return,
                        InitDbsError::MongoClientWithOptions(_) => return,
                        InitDbsError::MongoCollectionCountDocuments(_) => return,
                        InitDbsError::MongoCollectionIsNotEmpty((_, _)) => return,
                        InitDbsError::MongoCollectionInsertMany(_) => return,
                        InitDbsError::PostgresLoadingProvidersLinkParts(_) => return,
                        InitDbsError::PostgresProvidersLinkPartsIsNotEmpty(_) => (),
                        InitDbsError::PostgresInsertPosts(_) => return,
                        InitDbsError::PostgresEstablishConnection(_) => return,
                    }
                }
            }
            if ProviderKind::get_enabled_providers_vec().is_empty() {
                print_colorful_message(
                    None,
                    PrintType::WarningLow,
                    file!().to_string(),
                    line!().to_string(),
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
        file!().to_string(),
        line!().to_string(),
        format!("main done in {} seconds", time.elapsed().as_secs()),
    );
}
