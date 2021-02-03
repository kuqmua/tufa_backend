use super::check_link_metainfo_structures::HandledReachProviderStatusInfo;

pub fn fetch_link(
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
