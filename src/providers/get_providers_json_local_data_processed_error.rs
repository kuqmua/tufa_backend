use crate::providers::provider_kind_enum::ProviderKind;

use std::collections::HashMap;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(Debug)]
pub enum GetProvidersJsonLocalDataProcessedError {
    SerdeJsonErrors(Vec<serde_json::Error>),
    StdIoError(std::io::Error),
}