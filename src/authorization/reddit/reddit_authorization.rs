use roux::util::error::RouxError;
use roux::Reddit;

use crate::overriding::prints::print_error_red;

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
        Ok(_) => true,
        Err(roux_error_instans) => {
            match roux_error_instans {
                RouxError::Network(error_instans) => {
                    if let Some(eshe_errorishe) = error_instans.get_ref() {
                        print_error_red(
                            file!().to_string(),
                            line!().to_string(),
                            eshe_errorishe.to_string(),
                        )
                    }
                }
                _ => print_error_red(
                    file!().to_string(),
                    line!().to_string(),
                    "todo RouxError enum error".to_string(),
                ), //TODO
            }
            false
        }
    }
}
