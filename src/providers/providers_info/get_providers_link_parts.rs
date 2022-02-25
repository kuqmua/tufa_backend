use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::helpers::resource::Resource;

use crate::mongo_integration::mongo_get_providers_link_parts::mongo_get_providers_link_parts;
use crate::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsError;

// use crate::postgres_integration::postgres_get_providers_link_parts::postgres_get_providers_link_parts;
// use crate::postgres_integration::postgres_get_providers_link_parts::PostgresGetProviderLinksError;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct GetProvidersLinkPartsError {
    pub source: Box<GetProvidersLinkPartsErrorEnum>,
    where_was: WhereWas,
}

#[derive(Debug)]
pub enum GetProvidersLinkPartsErrorEnum {
    Local(GetLocalProvidersLinkPartsError),
    Mongodb(MongoGetProvidersLinkPartsError),
    // PostgreSql(PostgresGetProviderLinksError),
    PostgreSql,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn get_providers_link_parts(
    resource: &Resource,
) -> Result<HashMap<ProviderKind, Vec<String>>, GetProvidersLinkPartsError> {
    match resource {
        Resource::Local => match get_local_providers_link_parts().await {
            Err(error_hashmap) => Err(GetProvidersLinkPartsError {
                source: Box::new(GetProvidersLinkPartsErrorEnum::Local(error_hashmap)),
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(3 * 3600)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        Resource::Mongodb => match mongo_get_providers_link_parts().await {
            Err(e) => Err(GetProvidersLinkPartsError {
                source: Box::new(GetProvidersLinkPartsErrorEnum::Mongodb(e)),
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(3 * 3600)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        // Resource::PostgreSql => match postgres_get_providers_link_parts().await {
        //     Err(e) => Err(GetProvidersLinkPartsError {
        // source: Box::new(GetProvidersLinkPartsError::PostgreSql(e))
        // }),
        //     Ok(success_hashmap) => Ok(success_hashmap),
        // },
        Resource::PostgreSql => todo!(),
    }
}
