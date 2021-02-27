use crate::async_tokio_wrapper::tokio_wrapper;
use crate::check_net::check_link::check_link;
use crate::config::ENABLE_ERROR_PRINTS_HANDLE;
use crate::config::ENABLE_PRINTS_HANDLE;
use crate::config::STARTING_CHECK_URL;
use crate::overriding::prints::print_error_red;
extern crate num_cpus;

pub fn entry() {
    let cpus = num_cpus::get();
    if cpus > 1 {
        println!("We are on a multicore system with {} CPUs", cpus);
    } else {
        println!("We are on a single core system");
    }

    if check_link(STARTING_CHECK_URL).0 {
        if ENABLE_PRINTS_HANDLE {
            let its_all_good_message = "server can reach ".to_string() + STARTING_CHECK_URL;
            println!("{}", its_all_good_message);
        }
        tokio_wrapper();
    } else if ENABLE_ERROR_PRINTS_HANDLE {
        let its_not_good_message = "server cannot reach ".to_string() + STARTING_CHECK_URL;
        print_error_red(
            file!().to_string(),
            line!().to_string(),
            its_not_good_message,
        )
    }
}
