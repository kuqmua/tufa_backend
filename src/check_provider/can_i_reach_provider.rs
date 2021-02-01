use crate::overriding::prints::print_error_red;
use reqwest::Client;

#[tokio::main]
pub async fn reach_provider(url: String) -> bool {
    let client = Client::new();
    let response = client.get(&url).send().await; //body не нужно ток статус
    match response {
        Ok(resp) => {
            let reddit_string_status = resp.status().to_string();
            if reddit_string_status == "200 OK" {
                println!("{} status: 200 OK", url);
                true
            } else {
                let error: String = url + " status: {}" + &reddit_string_status;
                print_error_red(file!().to_string(), line!().to_string(), error);
                false
            }
        }
        Err(error) => {
            print_error_red(file!().to_string(), line!().to_string(), error.to_string());
            false
        }
    }
}
