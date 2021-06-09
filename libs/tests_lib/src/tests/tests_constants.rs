pub const CONFIG_FILE_EXTENSION: &str = ".toml";
pub static VECTOR_OF_MODES: &[&str] = &["Development", "Production", "Testing"];
//its important to have EXACT copy without spaces or Line feed character
pub const USER_CREDENTIALS_CONTENT: &str = r#"[github_authorization]
github_name = "example"
github_token = "example"

[reddit_authorization]
reddit_user_agent = "example"
reddit_client_id = "example"
reddit_client_secret = "example"
reddit_username = "example"
reddit_password = "example""#;
