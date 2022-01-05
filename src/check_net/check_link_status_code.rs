use std::fmt;

use reqwest::Error;

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct CheckLinkStatusCodeError(
    /// check link status code error `{0}`
    #[source]
    Box<Error>,
);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_link_status_code(link: &str) -> Result<reqwest::StatusCode, CheckLinkStatusCodeError> {
    let client = reqwest::blocking::Client::new();
    let res = reqwest::blocking::Client::head(&client, link).send()?;
    Ok(res.status())
}
