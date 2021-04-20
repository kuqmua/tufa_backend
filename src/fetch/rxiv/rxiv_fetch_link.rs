use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::overriding::prints::print_error_red;
use std::time::Instant;
//async
// let res = reqwest::get(link).await?;
//NOT WORKING FOR SOME REASON (returning 404)
// let client = reqwest::blocking::Client::new();
// let res = reqwest::blocking::Client::head(&client, link).send()?;
// let mut result_tuplefff: (bool, HandledReachProviderStatusInfo) =
//     (false, HandledReachProviderStatusInfo::Initialized);
// if res.status() == reqwest::StatusCode::OK {
//     println!("fetch_link res.status() ok");
//     result_tuplefff = (true, HandledReachProviderStatusInfo::Success)
// } else {
//     println!("fetch_link res.status() not ok");
//     result_tuplefff.1 = HandledReachProviderStatusInfo::ResStatusError(res.status());
// }
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
