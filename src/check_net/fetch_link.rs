#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn fetch_link(link: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = reqwest::blocking::Client::head(&client, link).send()?;
    Ok(res.status())
}
