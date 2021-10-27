use crate::providers::provider_kind_enum::ProviderKind;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Resource {
    Local,
    Mongodb {
        providers_string_into_enum_hashmap: HashMap<&'static str, ProviderKind>,
    },
    PostgreSql,
}
