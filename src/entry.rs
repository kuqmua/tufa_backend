use crate::async_tokio_wrapper::tokio_wrapper;
use crate::check_net::check_link::check_link;
use crate::config::ENABLE_ERROR_PRINTS_HANDLE;
use crate::config::ENABLE_PRINTS_HANDLE;
use crate::overriding::prints::print_error_red;

pub fn entry() {
    if check_link("https://www.google.com/").0 {
        if ENABLE_PRINTS_HANDLE {
            println!("server can reach https://www.google.com/");
        }
        tokio_wrapper();
    } else if ENABLE_ERROR_PRINTS_HANDLE {
        print_error_red(
            file!().to_string(),
            line!().to_string(),
            "server cannot reach https://www.google.com/".to_string(),
        )
    }
}
