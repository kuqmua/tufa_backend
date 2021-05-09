use crate::async_tokio_wrapper::async_tokio_wrapper;
use std::time::Instant;

use crate::check_net::check_link::check_link;
use crate::overriding::prints::print_error_red;

extern crate num_cpus;

use crate::get_project_information::get_config::get_config_information::get_config_information;

pub fn entry() {
    let option_of_config = get_config_information();
    match option_of_config {
        Some(config) => {
            let time = Instant::now();
            let cpus = num_cpus::get();

            if config.params.enable_prints_handle {
                if cpus > 1 {
                    println!("We are on a multicore system with {} CPUs", cpus);
                } else {
                    println!("We are on a single core system");
                }
            }
            let enable_common_time_measurement = config.params.enable_common_time_measurement; //need to be different variable cuz move happpens
            if check_link(
                &config.params.starting_check_link,
                config.params.enable_error_prints_handle,
            )
            .0
            {
                if config.params.enable_prints_handle {
                    let its_all_good_message =
                        "server can reach ".to_string() + &config.params.starting_check_link;
                    println!("{}", its_all_good_message);
                }
                async_tokio_wrapper(config);
            } else if config.params.enable_error_prints_handle {
                let its_not_good_message =
                    "server cannot reach ".to_string() + &config.params.starting_check_link;
                print_error_red(
                    file!().to_string(),
                    line!().to_string(),
                    its_not_good_message,
                )
            }

            if enable_common_time_measurement {
                println!("main done in {} seconds", time.elapsed().as_secs());
            }
        }
        None => print_error_red(
            file!().to_string(),
            line!().to_string(),
            "cannot get config information".to_string(),
        ),
    }
}
