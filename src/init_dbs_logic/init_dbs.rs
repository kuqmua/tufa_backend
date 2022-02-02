use super::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use super::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;

#[derive(Debug)]
pub struct InitDbsError {
    pub source: Box<InitDbsErrorEnum>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}
#[derive(Debug)]
pub enum InitDbsErrorEnum {
    InitDbsProvidersLinkParts(InitDbsProvidersLinkPartsError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    match init_dbs_with_providers_link_parts().await {
        Err(e) => Err(InitDbsError {
            source: Box::new(InitDbsErrorEnum::InitDbsProvidersLinkParts(e)),
            file: file!(),
            line: line!(),
            column: column!(),
        }),
        Ok(_) => Ok(()),
    }
}
