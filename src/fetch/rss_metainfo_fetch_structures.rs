use chrono::Local;
use serde_json::{Value, json};
use strum_macros::Display;

use crate::providers::provider_kind_enum::ProviderKind;

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
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn into_json_with_link_and_provider_kind(
    link: &str,
    no_items_error: &NoItemsError,
    provider_kind: &ProviderKind,
) -> Value {
    match no_items_error {
        NoItemsError::ThereIsTag(tag) => {
            json!({
                "error_kind": NoItemsError::get_stringified_kind(no_items_error),
                "link": link,
                "tag": tag,
                "part_of": ProviderKind::get_string_name(*provider_kind),
                "date": Local::now().to_string()
            })
        }
        NoItemsError::ConversionFromStrError(string, error) => json!({
            "error_kind": NoItemsError::get_stringified_kind(no_items_error),
            "link": link,
            "string": string,
            "error": error,
            "part_of": ProviderKind::get_string_name(*provider_kind),
            "date": Local::now().to_string()
        }),
        NoItemsError::NoTag(tag) => json!({
            "error_kind": NoItemsError::get_stringified_kind(no_items_error),
            "link": link,
            "tag": tag,
            "part_of": ProviderKind::get_string_name(*provider_kind),
            "date": Local::now().to_string()
        }),
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
