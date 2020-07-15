#[path = "reddit/authorization_info.rs"]
mod authorization_info;
#[path = "reddit/reddit_authorization.rs"]
mod reddit_authorization;

pub fn all_providers_authorization() {
    reddit_authorization::reddit_authorization(
        authorization_info::REDDIT_USER_AGENT,
        authorization_info::REDDIT_CLIENT_ID,
        authorization_info::REDDIT_CLIENT_SECRET,
        authorization_info::REDDIT_USERNAME,
        authorization_info::REDDIT_PASSWORD,
    );
}
