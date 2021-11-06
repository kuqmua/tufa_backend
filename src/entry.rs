use crate::async_tokio_wrapper::async_tokio_wrapper;
use std::time::Instant;

use crate::check_net::check_link::check_link;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::check_net::check_net_wrapper::check_net_wrapper;

extern crate num_cpus;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn entry() {
    let time = Instant::now();
    if CONFIG.params.enable_prints {
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
    println!("-------------- {}", check_net_wrapper());
    //todo: add block to check links to databases
    if check_link(&CONFIG.params.starting_check_link).0 {
        if CONFIG.params.enable_prints {
            print_colorful_message(
                None,
                PrintType::Info,
                file!().to_string(),
                line!().to_string(),
                format!("server can reach {}", &CONFIG.params.starting_check_link),
            );
        }
        async_tokio_wrapper();
    } else {
        print_colorful_message(
            None,
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!("server cannot reach {}", &CONFIG.params.starting_check_link),
        );
    }
    if CONFIG.params.enable_time_measurement {
        print_colorful_message(
            None,
            PrintType::Info,
            file!().to_string(),
            line!().to_string(),
            format!("main done in {} seconds", time.elapsed().as_secs()),
        );
    }
}
