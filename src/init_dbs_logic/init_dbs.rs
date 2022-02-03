use super::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use super::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;

#[derive(Debug)]
pub struct InitDbsError {
    pub source: Box<InitDbsErrorEnum>,
}
#[derive(Debug)]
pub enum InitDbsErrorEnum {
    InitDbsProvidersLinkParts {
        source: InitDbsProvidersLinkPartsError,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    if let Err(e) = init_dbs_with_providers_link_parts().await {
        return Err(InitDbsError {
            source: Box::new(InitDbsErrorEnum::InitDbsProvidersLinkParts{
                source: e,
                file: file!(),
                line: line!(),
                column: column!(),
            }),
        });
    }
    Ok(())
}
