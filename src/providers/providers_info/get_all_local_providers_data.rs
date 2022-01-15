use std::collections::HashMap;

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::get_local_data::ProviderGetLocalDataError;

use crate::traits::provider_kind_trait::ProviderKindTrait;

#[deny(clippy::indexing_slicing)]
pub async fn get_all_local_providers_data(
) -> Result<HashMap<ProviderKind, Vec<String>>, HashMap<ProviderKind, ProviderGetLocalDataError>> {
    let futures_vec = ProviderKind::get_enabled_providers_vec()
        .into_iter()
        .map(|pk| async move { (pk, ProviderKind::get_local_data(pk).await) });
    let result_vec = join_all(futures_vec).await;
    let mut errors_hashmap: HashMap<ProviderKind, ProviderGetLocalDataError> = HashMap::new();
    let mut success_hashmap: HashMap<ProviderKind, Vec<String>> =
        HashMap::with_capacity(ProviderKind::get_enabled_providers_vec().len());
    for (pk, result) in result_vec {
        match result {
            Err(e) => {
                errors_hashmap.insert(pk, e);
            }
            Ok(vec) => {
                success_hashmap.insert(pk, vec);
            }
        }
    }
    if !errors_hashmap.is_empty() {
        return Err(errors_hashmap);
    }
    Ok(success_hashmap)
}
