use std::fmt;

use crate::providers::provider_kind_enum::ProviderKind;

impl fmt::Display for ProviderKind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}
