#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
    pub params: Params,
    pub enable_providers: EnableProvidersStruct,
    pub reddit_authorization: RedditAuthorization,
    pub links: Links,
    pub enable_prints: EnablePrints,
    pub enable_warning_prints: EnableWarningPrints,
    pub enable_error_prints: EnableErrorPrints,
    pub enable_cleaning_warning_logs_directory: EnableCleaningWarningLogsDirectory,
    pub enable_time_measurement: EnableTimeMeasurement,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersStruct {
    pub enable_arxiv: bool,
    pub enable_biorxiv: bool,
    pub enable_habr: bool,
    pub enable_medrxiv: bool,
    pub enable_reddit: bool,
    pub enable_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditAuthorization {
    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Links {
    pub starting_check_link: String,
    pub arxiv_link: String,
    pub biorxiv_link: String,
    pub habr_link: String,
    pub medrxiv_link: String,
    pub reddit_link: String,
    pub twitter_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnablePrints {
    pub enable_prints_arxiv: bool,
    pub enable_prints_biorxiv: bool,
    pub enable_prints_habr: bool,
    pub enable_prints_medrxiv: bool,
    pub enable_prints_reddit: bool,
    pub enable_prints_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableWarningPrints {
    pub enable_warning_prints_for_arxiv: bool,
    pub enable_warning_prints_for_biorxiv: bool,
    pub enable_warning_prints_for_habr: bool,
    pub enable_warning_prints_for_medrxiv: bool,
    pub enable_warning_prints_for_reddit: bool,
    pub enable_warning_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableErrorPrints {
    pub enable_error_prints_for_arxiv: bool,
    pub enable_error_prints_for_biorxiv: bool,
    pub enable_error_prints_for_habr: bool,
    pub enable_error_prints_for_medrxiv: bool,
    pub enable_error_prints_for_reddit: bool,
    pub enable_error_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableCleaningWarningLogsDirectory {
    pub enable_cleaning_warning_logs_directory_for_arxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_habr: bool,
    pub enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_reddit: bool,
    pub enable_cleaning_warning_logs_directory_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableTimeMeasurement {
    pub enable_arxiv_time_measurement: bool,
    pub enable_biorxiv_time_measurement: bool,
    pub enable_habr_time_measurement: bool,
    pub enable_medrxiv_time_measurement: bool,
    pub enable_reddit_time_measurement: bool,
    pub enable_twitter_time_measurement: bool,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    pub enable_all_providers: bool,
    pub enable_all_providers_prints: bool,
    pub enable_warning_prints_for_all_providers: bool,
    pub enable_error_prints_for_all_providers: bool,
    pub enable_all_cleaning_warning_logs_directory: bool,
    pub enable_prints_handle: bool,
    pub enable_error_prints_handle: bool,
    pub warning_logs_directory_name: String,
    pub enable_all_time_measurement: bool,
    pub enable_common_time_measurement: bool,
}
