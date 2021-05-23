use crate::get_project_information::get_config::get_config_information::CONFIG;

#[test]
fn check_compromised_reddit_user_agent() {
    let reddit_user_agent = &CONFIG.reddit_authorization.reddit_user_agent;
    assert_eq!(reddit_user_agent, "example");
}

#[test]
fn check_compromised_reddit_client_id() {
    let reddit_client_id = &CONFIG.reddit_authorization.reddit_client_id;
    assert_eq!(reddit_client_id, "example");
}
#[test]
fn check_compromised_reddit_client_secret() {
    let reddit_client_secret = &CONFIG.reddit_authorization.reddit_client_secret;
    assert_eq!(reddit_client_secret, "example");
}
#[test]
fn check_compromised_reddit_username() {
    let reddit_username = &CONFIG.reddit_authorization.reddit_username;
    assert_eq!(reddit_username, "example");
}
#[test]
fn check_compromised_reddit_password() {
    let reddit_password = &CONFIG.reddit_authorization.reddit_password;
    assert_eq!(reddit_password, "example");
}
