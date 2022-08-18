use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsErrorEnum;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

#[derive(Debug)]
pub enum InitTablesEnumError {
    ProvidersLinkParts {
        source: InitDbsProvidersLinkPartsErrorEnum,
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
    pub async fn init(&self) -> Result<(), Box<InitTablesEnumError>> {
        match self {
            InitTablesEnum::ProvidersLinkParts => {
                if let Err(e) = init_dbs_with_providers_link_parts().await {
                    return Err(Box::new(InitTablesEnumError::ProvidersLinkParts {
                        source: *e,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
