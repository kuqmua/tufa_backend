use reqwest::Client;
use tokio;
#[path = "../../../override_prints.rs"]
mod override_prints;

#[tokio::main]
pub async fn can_i_reach_provider(url: String) -> bool {
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
                override_prints::print_error_red(file!().to_string(), line!().to_string(), error);
                return false;
            }
        }
        Err(error) => {
            override_prints::print_error_red(
                file!().to_string(),
                line!().to_string(),
                error.to_string(),
            );
            return false;
        }
    }
}
