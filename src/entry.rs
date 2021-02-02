use crate::check_provider::can_i_reach_provider::reach_provider;
// use crate::overriding::prints::print_error_red;
use crate::threads_parts::threads_parts;

pub fn entry() {
    if reach_provider("https://www.google.com/").0 {
        println!("i can reach https://www.google.com/");
        fuck();
    } else {
        println!("i cannot reach https://www.google.com/");
    }
}
#[tokio::main]
async fn fuck() {
    threads_parts().await;
}
