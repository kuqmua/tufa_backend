use crate::fetch::rss_metainfo_fetch_structures::RssFetchLinkError;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

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
#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_fetch_link(link: &str, time: Instant) -> Result<String, RssFetchLinkError> {
    let res = reqwest::blocking::get(link)?;
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        file!().to_string(),
        line!().to_string(),
        format!(
            "fetch in {}.{}ms... status {}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis() / 10,
            res.status(),
        ),
    );
    if !(res.status() == reqwest::StatusCode::OK) {
        print_colorful_message(
            None,
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!("LINK: {} RES.STATUS: {}", link, res.status()),
        );
        return Err(RssFetchLinkError::StatusCode(res.status()));
    }
    Ok(res.text()?)
}
