use crate::helpers::fetch::fetch_link_error::FetchLinkError;
use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn blocking_fetch_link(link: &str) -> Result<String, FetchLinkError> {
    match reqwest::blocking::get(link) {
        Err(e) => {
            return Err(FetchLinkError {
                source: Box::new(FetchLinkErrorEnum::ReqwestBlockingGet(e)),
                line: format!("{} {}", line!().to_string(), file!().to_string())
            })
        }
        Ok(res) => {
            if res.status() != reqwest::StatusCode::OK {
                return Err(FetchLinkError {
                    source: Box::new(FetchLinkErrorEnum::StatusCode(res.status())),
                    line: format!("{} {}", line!().to_string(), file!().to_string())
                });
            }
            match res.text() {
                Err(e) => {
                    return Err(FetchLinkError {
                        source: Box::new(FetchLinkErrorEnum::ResponseText(e)),
                        line: format!("{} {}", line!().to_string(), file!().to_string())
                    })
                }
                Ok(text) => Ok(text),
            }
        }
    }
}
