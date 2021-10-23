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
