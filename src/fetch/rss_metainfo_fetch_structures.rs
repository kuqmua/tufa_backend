use strum_macros::Display;

#[derive(Debug, Clone)] //Debug only for prints
pub enum NoItemsError {
    ThereIsTag(String),
    ConversionFromStrError(String, String),
    NoTag(String),
}

#[derive(Debug, Display)] //Debug only for prints
pub enum RssFetchLinkError {
    ReqwestBlockingGet(reqwest::Error),
    StatusCode(reqwest::StatusCode),
}

impl From<reqwest::Error> for RssFetchLinkError {
    fn from(e: reqwest::Error) -> Self {
        RssFetchLinkError::ReqwestBlockingGet(e)
    }
}
