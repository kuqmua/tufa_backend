use reqwest::Client;
use tokio;

use crate::override_prints::override_prints::print_error_red;

#[tokio::main]
pub async fn reach_provider(url: String) -> bool {
    let client = Client::new();
    let response = client.get(&url).send().await;
    match response {
        Ok(resp) => {
            let reddit_string_status = resp.status().to_string();
            if reddit_string_status == "200 OK" {
                println!("{} status: 200 OK", url);
                return true;
            } else {
                let error: String = url + " status: {}" + &reddit_string_status;
                print_error_red(file!().to_string(), line!().to_string(), error);
                return false;
            }
        }
        Err(error) => {
            print_error_red(file!().to_string(), line!().to_string(), error.to_string());
            return false;
        }
    }
}
