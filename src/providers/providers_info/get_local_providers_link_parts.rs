use std::collections::HashMap;

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::get_link_parts_from_local_json_file::GetLinkPartsFromLocalJsonFileErrorEnum;

use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct GetLocalProvidersLinkPartsError {
    pub source: Box<HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileErrorEnum>>,
    where_was: WhereWas,
}

#[deny(clippy::indexing_slicing)]
pub async fn get_local_providers_link_parts(
) -> Result<HashMap<ProviderKind, Vec<String>>, GetLocalProvidersLinkPartsError> {
    let result_vec = join_all(
        ProviderKind::get_enabled_providers_vec()//maybe its not exactly correct
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
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        });
    }
    Ok(success_hashmap)
}
