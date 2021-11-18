use strum_macros::Display;

#[derive(Debug, Clone)] //Debug only for prints
pub enum NoItemsError {
    ThereIsTag(String),
    ConversionFromStrError(String, String),
    NoTag(String),
}

impl NoItemsError {
    pub fn get_stringified_kind(error: &NoItemsError) -> &'static str {
        match error {
            NoItemsError::ThereIsTag(_) => stringify!(NoItemsError::ThereIsTag),
            NoItemsError::ConversionFromStrError(_, _) => stringify!(NoItemsError::ConversionFromStrError),
            NoItemsError::NoTag(_) => stringify!(NoItemsError::NoTag),
        }
    }
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
