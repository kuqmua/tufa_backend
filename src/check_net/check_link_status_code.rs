#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_link_status_code(link: &str) -> Result<reqwest::StatusCode, Box<reqwest::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = reqwest::blocking::Client::head(&client, link).send()?;
    Ok(res.status())
}
