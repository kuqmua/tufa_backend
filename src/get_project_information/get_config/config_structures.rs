use config::{Config, ConfigError, File};
use std::fmt;
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct ConfigStruct {
    pub params: Params,
    pub enable_providers: EnableProvidersStruct,
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    pub links: Links,
    pub enable_prints: EnablePrints,
    pub enable_warning_prints: EnableWarningPrints,
    pub enable_error_prints: EnableErrorPrints,
    pub enable_cleaning_warning_logs_directory: EnableCleaningWarningLogsDirectory,
    pub enable_time_measurement: EnableTimeMeasurement,
    pub env: Env,
}
const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError> {
        // RUN_ENV=Testing cargo run
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut config = Config::new();
        config.set("env", env.clone())?;
        config.merge(File::with_name(CONFIG_FILE_PATH))?;
        config.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;
        config.try_into()
    }
    pub fn test_values(mode: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.set("env", mode)?;
        config.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, mode)))?;
        config.try_into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersStruct {
    pub enable_arxiv: bool,
    pub enable_biorxiv: bool,
    pub enable_github: bool,
    pub enable_habr: bool,
    pub enable_medrxiv: bool,
    pub enable_reddit: bool,
    pub enable_twitter: bool,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GithubAuthorization {
    pub github_token: String,
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
    pub github_link: String,
    pub habr_link: String,
    pub medrxiv_link: String,
    pub reddit_link: String,
    pub twitter_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnablePrints {
    pub enable_prints_arxiv: bool,
    pub enable_prints_biorxiv: bool,
    pub enable_prints_github: bool,
    pub enable_prints_habr: bool,
    pub enable_prints_medrxiv: bool,
    pub enable_prints_reddit: bool,
    pub enable_prints_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableWarningPrints {
    pub enable_warning_prints_for_arxiv: bool,
    pub enable_warning_prints_for_biorxiv: bool,
    pub enable_warning_prints_for_github: bool,
    pub enable_warning_prints_for_habr: bool,
    pub enable_warning_prints_for_medrxiv: bool,
    pub enable_warning_prints_for_reddit: bool,
    pub enable_warning_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableErrorPrints {
    pub enable_error_prints_for_arxiv: bool,
    pub enable_error_prints_for_biorxiv: bool,
    pub enable_error_prints_for_github: bool,
    pub enable_error_prints_for_habr: bool,
    pub enable_error_prints_for_medrxiv: bool,
    pub enable_error_prints_for_reddit: bool,
    pub enable_error_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableCleaningWarningLogsDirectory {
    pub enable_cleaning_warning_logs_directory_for_arxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_github: bool,
    pub enable_cleaning_warning_logs_directory_for_habr: bool,
    pub enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_reddit: bool,
    pub enable_cleaning_warning_logs_directory_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableTimeMeasurement {
    pub enable_arxiv_time_measurement: bool,
    pub enable_biorxiv_time_measurement: bool,
    pub enable_github_time_measurement: bool,
    pub enable_habr_time_measurement: bool,
    pub enable_medrxiv_time_measurement: bool,
    pub enable_reddit_time_measurement: bool,
    pub enable_twitter_time_measurement: bool,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub enable_all_providers: bool,
    pub enable_all_providers_prints: bool,
    pub enable_warning_prints_for_all_providers: bool,
    pub enable_error_prints_for_all_providers: bool,
    pub enable_all_cleaning_warning_logs_directory: bool,
    pub enable_prints_handle: bool,
    pub enable_error_prints_handle: bool,
    pub enable_all_time_measurement: bool,
    pub enable_common_time_measurement: bool,
}

#[derive(Clone, Debug, serde_derive::Deserialize, PartialEq, serde_derive::Serialize)]
pub enum Env {
    Default,
    Development,
    Testing,
    Production,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Default => write!(f, "Default"),
            Env::Development => write!(f, "Development"),
            Env::Production => write!(f, "Production"),
            Env::Testing => write!(f, "Testing"),
        }
    }
}

impl From<&str> for Env {
    fn from(env: &str) -> Self {
        match env {
            "Default" => Env::Default,
            "Development" => Env::Development,
            "Production" => Env::Production,
            "Testing" => Env::Testing,
            _ => Env::Default,
        }
    }
}
