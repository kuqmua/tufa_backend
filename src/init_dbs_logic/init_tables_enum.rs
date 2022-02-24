use strum_macros::EnumIter;

use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;

use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;

use crate::helpers::where_was::WhereWas;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

#[derive(Debug)]
pub enum InitDbsErrorEnum {
    InitDbsProvidersLinkParts {
        source: InitDbsProvidersLinkPartsError,
        where_was: WhereWas,
    },
}

impl InitTablesEnum {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn init(&self) -> Result<(), Box<InitDbsErrorEnum>> {
        match self {
            InitTablesEnum::ProvidersLinkParts => {
                if let Err(e) = init_dbs_with_providers_link_parts().await {
                    return Err(Box::new(InitDbsErrorEnum::InitDbsProvidersLinkParts {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }));
                }
            }
        }
        Ok(())
    }
}
