use config::{Config, ConfigError, File};
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct UserCredentialsStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
}
const CONFIG_FILE_PREFIX: &str = "./config/";
const USER_CREDENTIALS_FILE_NAME: &str = "User_credentials";

impl UserCredentialsStruct {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name(&format!(
            "{}{}",
            CONFIG_FILE_PREFIX, USER_CREDENTIALS_FILE_NAME
        )))?;
        config.try_into()
    }
    pub fn test_values(mode: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name(&format!(
            "{}{}",
            CONFIG_FILE_PREFIX, USER_CREDENTIALS_FILE_NAME
        )))?;
        config.try_into()
    }
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
