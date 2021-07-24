use crate::check_net::check_link_metainfo_structures::HandledReachProviderStatusInfo;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn fetch_link(
    link: &str,
) -> Result<(bool, HandledReachProviderStatusInfo), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = reqwest::blocking::Client::head(&client, link).send()?;
    let mut result_tuple: (bool, HandledReachProviderStatusInfo) =
        (false, HandledReachProviderStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        result_tuple = (true, HandledReachProviderStatusInfo::Success)
    } else {
        result_tuple.1 = HandledReachProviderStatusInfo::ResStatusError(res.status());
    }
    Ok(result_tuple)
}
