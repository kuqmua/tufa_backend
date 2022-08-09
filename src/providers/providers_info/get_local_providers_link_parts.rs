use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::providers::provider_kind::functions::get_link_parts_from_local_json_file::GetLinkPartsFromLocalJsonFileErrorEnum;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GetLocalProvidersLinkPartsError {
    pub source: Box<HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileErrorEnum>>,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn get_local_providers_link_parts(
) -> Result<HashMap<ProviderKind, Vec<String>>, GetLocalProvidersLinkPartsError> {
    let result_vec = join_all(
        ProviderKind::get_enabled_providers_vec() //maybe its not exactly correct
            .into_iter()
            .map(|pk| async move {
                (
                    pk,
                    ProviderKind::get_link_parts_from_local_json_file(pk).await,
                )
            }),
    )
    .await;
    let mut errors_hashmap: HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileErrorEnum> =
        HashMap::new();
    let mut success_hashmap: HashMap<ProviderKind, Vec<String>> =
        HashMap::with_capacity(result_vec.len());
    for (pk, result) in result_vec {
        match result {
            Err(e) => {
                errors_hashmap.insert(pk, *e);
            }
            Ok(vec) => {
                success_hashmap.insert(pk, vec);
            }
        }
    }
    if !errors_hashmap.is_empty() {
        return Err(GetLocalProvidersLinkPartsError {
            source: Box::new(errors_hashmap),
            where_was: WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                None,
            ),
        });
    }
    Ok(success_hashmap)
}
