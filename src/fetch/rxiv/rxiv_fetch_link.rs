use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::overriding::prints::print_error_red;
use std::time::Instant;

pub fn rxiv_fetch_link(
    link: &str,
    key: &str,
    time: Instant,
    enable_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
) -> Result<(String, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    if enable_time_measurement {
        println!(
            "fetch in {}.{}ms... status {} for {}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis() / 10,
            res.status(),
            key
        );
    }
    let mut result_tuple: (String, HandledFetchStatusInfo) =
        ("".to_string(), HandledFetchStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        let res_to_text_result = res.text();
        match res_to_text_result {
            Ok(norm) => result_tuple = (norm, HandledFetchStatusInfo::Success),
            Err(e) => {
                result_tuple.1 = HandledFetchStatusInfo::ResToTextError(e.to_string());
                let error_message = format!(
                    "KEY: {} LINK: {} ResToTextError...(decided to not show)",
                    key, link
                );
                if enable_error_prints {
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
            }
        }
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        let error_message = format!("KEY: {} LINK: {} RES.STATUS: {}", key, link, res.status());
        if enable_error_prints {
            print_error_red(file!().to_string(), line!().to_string(), error_message);
        }
    }
    Ok(result_tuple)
}
