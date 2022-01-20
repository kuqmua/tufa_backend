use std::collections::HashMap;

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::get_link_parts_from_local_file::ProviderGetLocalDataError;

use crate::traits::provider_kind_trait::ProviderKindTrait;

use std::fmt;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct GetLocalProvidersLinkPartsError {
    pub source: Box<GetLocalProvidersLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum GetLocalProvidersLinkPartsErrorEnum {
    EnabledProvidersVecIsEmpty,
    GetProvidersLinkPartsFromFile(HashMap<ProviderKind, ProviderGetLocalDataError>),
}

#[deny(clippy::indexing_slicing)]
pub async fn get_local_providers_link_parts(
) -> Result<HashMap<ProviderKind, Vec<String>>, GetLocalProvidersLinkPartsError> {
    let enabled_providers_vec = ProviderKind::get_dbs_initialization_enabled_vec();
    if enabled_providers_vec.is_empty() {
        return Err(GetLocalProvidersLinkPartsError {
            source: Box::new(GetLocalProvidersLinkPartsErrorEnum::EnabledProvidersVecIsEmpty),
        });
    }
    let futures_vec = enabled_providers_vec
        .into_iter()
        .map(|pk| async move { (pk, ProviderKind::get_link_parts_from_local_file(pk).await) });
    let result_vec = join_all(futures_vec).await;
    let mut errors_hashmap: HashMap<ProviderKind, ProviderGetLocalDataError> = HashMap::new();
    let mut success_hashmap: HashMap<ProviderKind, Vec<String>> =
        HashMap::with_capacity(result_vec.len());
    for (pk, result) in result_vec {
        match result {
            Err(e) => {
                errors_hashmap.insert(pk, e);
            }
            Ok(vec) => {
                //todo check vec on empty
                success_hashmap.insert(pk, vec);
            }
        }
    }
    if !errors_hashmap.is_empty() {
        return Err(GetLocalProvidersLinkPartsError {
            source: Box::new(
                GetLocalProvidersLinkPartsErrorEnum::GetProvidersLinkPartsFromFile(errors_hashmap),
            ),
        });
    }
    Ok(success_hashmap)
}
