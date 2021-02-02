use crate::check_provider::can_i_reach_provider::reach_provider;
// use crate::overriding::prints::print_error_red;
use crate::threads_parts::threads_parts;
// use reqwest::Client;
// use tokio;

pub fn entry() {
    if reach_provider("https://www.google.com/").0 {
        println!("i can reach https://www.google.com/");
        // let client = Client::new();
        // let response = client.get("https://www.google.com/").send().await; //body не нужно ток статус
        // match response {
        //     Ok(resp) => {
        //         let reddit_string_status = resp.status().to_string();
        //         if reddit_string_status == "200 OK" {
        //             println!(" status: 200 OK");
        fuck();
    } else {
        println!("i cannot reach https://www.google.com/");
    }
    //             // return true;
    //         } else {
    //             let error: String = reddit_string_status;
    //             print_error_red(file!().to_string(), line!().to_string(), error);
    //             // return false;
    //         }
    //     }
    //     Err(error) => {
    //         print_error_red(file!().to_string(), line!().to_string(), error.to_string());
    //         // return false;
    //     }
    // }
}
#[tokio::main]
async fn fuck() {
    threads_parts().await;
}
// pub async fn entry() {
//     if reach_provider("https://www.google.com/").0 {
//         println!("i can reach https://www.google.com/");
//         threads_parts().await;
//     } else {
//         println!("i cannot reach https://www.google.com/");
//     }
// }
