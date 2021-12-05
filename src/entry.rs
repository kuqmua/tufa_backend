use crate::async_tokio_wrapper::async_tokio_wrapper;
use std::time::Instant;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::lazy_static_config::CONFIG;

extern crate num_cpus;

use crate::check_net::check_net_wrapper::check_net_wrapper;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn entry() {
    let time = Instant::now();
    if CONFIG.enable_prints {
        let cpus = num_cpus::get();
        if cpus > 1 {
            print_colorful_message(
                None,
                PrintType::Info,
                file!().to_string(),
                line!().to_string(),
                format!("We are on a multicore system with {} CPUs", cpus),
            );
        } else {
            print_colorful_message(
                None,
                PrintType::Info,
                file!().to_string(),
                line!().to_string(),
                "We are on a single core system".to_string(),
            );
        }
    }

    let is_all_available_result = check_net_wrapper();
    match is_all_available_result {
        Ok(_) => {
            async_tokio_wrapper();
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("check_net_wrapper error: {:#?}", e),
            );
            //do something with it
        }
    }
    //move time measument in some inner part coz it would be server here
    if CONFIG.enable_time_measurement {
        print_colorful_message(
            None,
            PrintType::Info,
            file!().to_string(),
            line!().to_string(),
            format!("main done in {} seconds", time.elapsed().as_secs()),
        );
    }
}
