use super::reach_link_meta_structures::HandledReachProviderStatusInfo;
use super::reach_link_meta_structures::UnhandledReachProviderInfo;

pub fn reach_provider(url: &str) -> (bool, UnhandledReachProviderInfo) {
    let fetch_result = fetch_link(url);
    let mut can_i: bool = false;
    let unhandled_info: UnhandledReachProviderInfo;
    match fetch_result {
        Ok(fetch_tuple_result) => {
            can_i = fetch_tuple_result.0;
            unhandled_info = UnhandledReachProviderInfo::Success;
        }
        Err(e) => {
            unhandled_info = UnhandledReachProviderInfo::Failure(e.to_string());
        }
    }
    (can_i, unhandled_info)
}
fn fetch_link(
    link: &str,
) -> Result<(bool, HandledReachProviderStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    let mut result_tuple: (bool, HandledReachProviderStatusInfo) =
        (false, HandledReachProviderStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        result_tuple = (true, HandledReachProviderStatusInfo::Success)
    } else {
        result_tuple.1 = HandledReachProviderStatusInfo::ResStatusError(res.status());
    }
    Ok(result_tuple)
}
