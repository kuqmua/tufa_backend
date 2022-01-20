use super::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;
use super::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;

#[derive(Debug)]
pub enum InitDbsError {
    InitDbsProvidersLinkParts(InitDbsProvidersLinkPartsError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    match init_dbs_with_providers_link_parts().await {
        Err(e) => Err(InitDbsError::InitDbsProvidersLinkParts(e)),
        Ok(_) => {
            Ok(())
        }
    }
}
