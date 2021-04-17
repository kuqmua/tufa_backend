use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::overriding::prints::print_error_red;
//async
pub fn twitter_check_provider_status_aka_rxiv_fetch_link(
    link: &str,
    enable_error_prints: bool,
) -> Result<(bool, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    // let res = reqwest::get(link).await?;

    let mut result_tuple: (bool, HandledFetchStatusInfo) =
        (false, HandledFetchStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        result_tuple.0 = true;
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        if enable_error_prints {
            let error_message = format!("{} {}", link, res.status());
            print_error_red(file!().to_string(), line!().to_string(), error_message);
        }
    }
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
    Ok(result_tuple)
}
