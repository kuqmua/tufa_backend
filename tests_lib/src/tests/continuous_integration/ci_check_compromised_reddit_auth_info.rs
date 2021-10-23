use crate::tests::tests_constants::USER_CREDENTIALS_DUMMY_HANDLE;
use crate::tests::tests_constants::REDDIT_USER_AGENT_ENV_NAME;
use crate::tests::tests_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::tests::tests_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::tests::tests_constants::REDDIT_USERNAME_ENV_NAME;
use crate::tests::tests_constants::REDDIT_PASSWORD_ENV_NAME;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    match std::env::var(REDDIT_USER_AGENT_ENV_NAME) {
        Ok(reddit_user_agent) => {
            if reddit_user_agent != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", REDDIT_USER_AGENT_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    reddit_user_agent
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_USER_AGENT_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_CLIENT_ID_ENV_NAME) {
        Ok(reddit_client_id) => {
            if reddit_client_id != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", REDDIT_CLIENT_ID_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    reddit_client_id
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_CLIENT_ID_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_CLIENT_SECRET_ENV_NAME) {
        Ok(rediit_client_secret) => {
            if rediit_client_secret != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", REDDIT_CLIENT_SECRET_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    rediit_client_secret
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_CLIENT_SECRET_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_USERNAME_ENV_NAME) {
        Ok(reddit_username) => {
            if reddit_username != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", REDDIT_USERNAME_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    reddit_username
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_USERNAME_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_PASSWORD_ENV_NAME) {
        Ok(reddit_password) => {
            if reddit_password != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", REDDIT_PASSWORD_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    reddit_password
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_PASSWORD_ENV_NAME, e);
        }   
    }
}
