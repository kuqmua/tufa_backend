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
    let cpus = num_cpus::get();
    if CONFIG.enable_prints {
        print_colorful_message(
            None,
            PrintType::Info,
            file!().to_string(),
            line!().to_string(),
            format!("We are on a multicore system with {} CPUs", cpus),
        );
    }
    if cpus <= 0 {
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
        //do something with it
        return
    }
    async_tokio_wrapper();
    //move time measument in some inner part coz it would be server here
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        file!().to_string(),
        line!().to_string(),
        format!("main done in {} seconds", time.elapsed().as_secs()),
    );
}
