// use crate::check_provider::can_i_reach_provider::wreach_provider;
use crate::override_prints::override_prints::print_error_red;
use crate::threads_parts::threads_parts;
use reqwest::Client;
use tokio;
#[tokio::main]
pub async fn entry() {
    let client = Client::new();
    let response = client.get("https://www.google.com/").send().await; //body не нужно ток статус
    match response {
        Ok(resp) => {
            let reddit_string_status = resp.status().to_string();
            if reddit_string_status == "200 OK" {
                println!(" status: 200 OK");
                threads_parts().await;
                // return true;
            } else {
                let error: String = reddit_string_status;
                print_error_red(file!().to_string(), line!().to_string(), error);
                // return false;
            }
        }
        Err(error) => {
            print_error_red(file!().to_string(), line!().to_string(), error.to_string());
            // return false;
        }
    }
}
