use crate::helpers::fetch::fetch_link_error::FetchLinkError;
use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::time::Instant;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn async_fetch_link(link: &str, time: Instant) -> Result<String, FetchLinkError> {
    match reqwest::get(link).await {
        Err(e) => return Err(FetchLinkError {
            source: Box::new(FetchLinkErrorEnum::ReqwestBlockingGet(e))
        }),
        Ok(res) => {
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
                return Err(FetchLinkError {
                    source: Box::new(FetchLinkErrorEnum::StatusCode(res.status()))
                });
            }
            match res.text().await {
                Err(e) => return Err(FetchLinkError {
                    source: Box::new(FetchLinkErrorEnum::ResponseText(e))
                }),
                Ok(text) => Ok(text),
            }
            
        },
    }
}