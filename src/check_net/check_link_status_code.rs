use std::fmt;

use reqwest::Error;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub struct CheckLinkStatusCodeError {
    /// check link status code error `{0}`
    pub source: Box<Error>,
}

impl fmt::Display for CheckLinkStatusCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Error> for CheckLinkStatusCodeError {
    fn from(error: Error) -> Self {
        CheckLinkStatusCodeError {
            source: Box::new(error),
        }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_link_status_code(link: &str) -> Result<reqwest::StatusCode, CheckLinkStatusCodeError> {
    let res = reqwest::blocking::Client::head(&reqwest::blocking::Client::new(), link).send()?;
    Ok(res.status())
}
