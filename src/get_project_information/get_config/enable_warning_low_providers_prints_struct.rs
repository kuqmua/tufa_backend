#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableWarningLowProvidersPrints {
    pub enable_warning_low_prints_for_arxiv: bool,
    pub enable_warning_low_prints_for_biorxiv: bool,
    pub enable_warning_low_prints_for_github: bool,
    pub enable_warning_low_prints_for_habr: bool,
    pub enable_warning_low_prints_for_medrxiv: bool,
    pub enable_warning_low_prints_for_reddit: bool,
    pub enable_warning_low_prints_for_twitter: bool,
}
