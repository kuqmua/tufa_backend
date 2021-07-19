use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

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
pub fn rss_fetch_link(
    link: &str,
    time: Instant,
    enable_error_prints: bool,
    enable_time_measurement: bool,
) -> Result<(String, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    if enable_time_measurement {
        println!(
            "fetch in {}.{}ms... status {}", // for {}
            time.elapsed().as_secs(),
            time.elapsed().as_millis() / 10,
            res.status(),
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
                let error_message =
                    format!("LINK: {} ResToTextError...(decided to not show)", link);
                if enable_error_prints {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        error_message,
                    );
                }
            }
        }
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        let error_message = format!("LINK: {} RES.STATUS: {}", link, res.status());
        if enable_error_prints {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                error_message,
            );
        }
    }
    Ok(result_tuple)
}
