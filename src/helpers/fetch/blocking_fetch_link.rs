use crate::fetch::rss_metainfo_fetch_structures::RssFetchLinkError;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::time::Instant;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn blocking_fetch_link(link: &str, time: Instant) -> Result<String, RssFetchLinkError> {
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
    if res.status() != reqwest::StatusCode::OK {
        return Err(RssFetchLinkError::StatusCode(res.status()));
    }
    Ok(res.text()?)
}
