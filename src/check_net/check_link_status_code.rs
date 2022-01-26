#[derive(Debug, BoxErrFromErrDerive)]
pub struct CheckLinkStatusCodeError {
    pub source: Box<reqwest::Error>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_link_status_code(
    link: &str,
) -> Result<reqwest::StatusCode, CheckLinkStatusCodeError> {
    let res = reqwest::get(link).await?;
    Ok(res.status())
}
