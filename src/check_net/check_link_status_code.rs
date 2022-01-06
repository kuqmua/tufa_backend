use std::fmt;

use reqwest::Error;

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct CheckLinkStatusCodeError {
    /// check link status code error `{0}`
    pub source: Box<Error>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_link_status_code(
    link: &str,
) -> Result<reqwest::StatusCode, CheckLinkStatusCodeError> {
    let res = reqwest::get(link).await?;
    Ok(res.status())
}
