use crate::providers::provider_kind_enum::ProviderKind;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableProvidersTimeMeasurement {
    pub enable_time_measurement_for_arxiv: bool,
    pub enable_time_measurement_for_biorxiv: bool,
    pub enable_time_measurement_for_github: bool,
    pub enable_time_measurement_for_habr: bool,
    pub enable_time_measurement_for_medrxiv: bool,
    pub enable_time_measurement_for_reddit: bool,
    pub enable_time_measurement_for_twitter: bool,
}

impl EnableProvidersTimeMeasurement {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_time_measurement_enabled(&self, provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => self.enable_time_measurement_for_arxiv,
            ProviderKind::Biorxiv => self.enable_time_measurement_for_biorxiv,
            ProviderKind::Github => self.enable_time_measurement_for_github,
            ProviderKind::Habr => self.enable_time_measurement_for_habr,
            ProviderKind::Medrxiv => self.enable_time_measurement_for_medrxiv,
            ProviderKind::Reddit => self.enable_time_measurement_for_reddit,
            ProviderKind::Twitter => self.enable_time_measurement_for_twitter,
        }
    }
}
