use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

impl InitTablesEnum {
    pub fn init(&self) {
        match self {
            InitTablesEnum::ProvidersLinkParts => todo!(),
        }
    }
}