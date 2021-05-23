use crate::get_project_information::get_config::get_config_information::CONFIG;

#[test]
fn it_works() {
    //     pub reddit_user_agent: String,
    // pub reddit_client_id: String,
    // pub reddit_client_secret: String,
    // pub reddit_username: String,
    // pub reddit_password: String,
    let f = &CONFIG.reddit_authorization.reddit_user_agent;
    assert_eq!(f, "example");
}
