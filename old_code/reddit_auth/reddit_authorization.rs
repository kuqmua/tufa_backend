// use roux::util::error::RouxError;
// use roux::Reddit;
// use tufa_common::config_mods::print_type::PrintType;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKind;

// #[deny(,   unwrap_used)]
// pub fn reddit_authorization(
//     user_agent: &str,
//     client_id: &str,
//     client_secret: &str,
//     username: &str,
//     password: &str,
// ) -> bool {
//     let reddit_authorization_result = Reddit::new(user_agent, client_id, client_secret)
//         .username(username)
//         .password(password)
//         .login();
//     match reddit_authorization_result {
//         Ok(_) => true,
//         Err(roux_error_instans) => {
//             match roux_error_instans {
//                 RouxError::Network(error_instans) => {
//                     if let Some(eshe_errorishe) = error_instans.get_ref() {
//                     } else {
//                     }
//                 }
//                 _ => (), //TODO
//             }
//             false
//         }
//     }
// }
