use roux::util::error::RouxError;
use roux::Reddit;

use crate::override_prints::override_prints::print_error_red;

pub fn reddit_authorization(
    user_agent: &str,
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) -> bool {
    let reddit_authorization_result = Reddit::new(user_agent, client_id, client_secret)
        .username(username)
        .password(password)
        .login();
    match reddit_authorization_result {
        Ok(_) => {
            return true;
        }
        Err(roux_error_instans) => {
            match roux_error_instans {
                RouxError::Network(error_instans) => match error_instans.get_ref() {
                    Some(eshe_errorishe) => print_error_red(
                        file!().to_string(),
                        line!().to_string(),
                        eshe_errorishe.to_string(),
                    ),
                    None => {}
                },
                _ => print_error_red(
                    file!().to_string(),
                    line!().to_string(),
                    "todo RouxError enum error".to_string(),
                ), //TODO
            }
            return false;
        }
    }
}
