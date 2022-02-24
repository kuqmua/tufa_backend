use strum_macros::EnumIter;

use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;

use crate::init_dbs_logic::init_dbs::InitDbsErrorEnum;

use crate::helpers::where_was::WhereWas;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

impl InitTablesEnum {
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
            },
        }
        Ok(())
    }
}