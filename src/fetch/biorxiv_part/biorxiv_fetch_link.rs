use super::biorxiv_metainfo_structures::HandledFetchStatusInfo;
use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::overriding::prints::print_error_red;
use std::time::Instant;

pub fn biorxiv_fetch_link(
    link: &str,
    key: &str,
    time: Instant,
) -> Result<(String, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    if ENABLE_PRINTS_BIORXIV {
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
                if ENABLE_ERROR_PRINTS_BIORXIV {
                    print_error_red(file!().to_string(), line!().to_string(), e.to_string());
                }
            }
        }
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        if ENABLE_ERROR_PRINTS_BIORXIV {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                res.status().to_string(),
            );
        }
    }
    Ok(result_tuple)
}
