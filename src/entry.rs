use crate::async_tokio_wrapper::async_tokio_wrapper;
use std::time::Instant;

use crate::check_net::check_link::check_link;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::get_project_information::get_config::config::CONFIG;

extern crate num_cpus;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();

    if CONFIG.params.enable_prints {
        if cpus > 1 {
            println!("We are on a multicore system with {} CPUs", cpus);
        } else {
            println!("We are on a single core system");
        }
    }
    let enable_common_time_measurement = CONFIG.params.enable_time_measurement; //need to be different variable cuz move happpens
    if check_link(
        &CONFIG.params.starting_check_link,
        CONFIG.params.enable_error_prints,
    )
    .0
    {
        if CONFIG.params.enable_prints {
            let its_all_good_message =
                "server can reach ".to_string() + &CONFIG.params.starting_check_link;
            println!("{}", its_all_good_message);
        }
        async_tokio_wrapper();
    } else if CONFIG.params.enable_error_prints {
        let warning_high_message =
            "server cannot reach ".to_string() + &CONFIG.params.starting_check_link;
        print_colorful_message(
            None,
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            warning_high_message,
        );
    }

    if enable_common_time_measurement {
        println!("main done in {} seconds", time.elapsed().as_secs());
    }
}
