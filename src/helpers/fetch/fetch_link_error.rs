use strum_macros::Display;

#[derive(Debug, Display)] //Debug only for prints
pub enum FetchLinkError {
    ReqwestBlockingGet(reqwest::Error),
    StatusCode(reqwest::StatusCode),
}

impl From<reqwest::Error> for FetchLinkError {
    fn from(e: reqwest::Error) -> Self {
        FetchLinkError::ReqwestBlockingGet(e)
    }
}
