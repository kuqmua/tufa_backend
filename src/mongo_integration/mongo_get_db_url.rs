use crate::config_mods::config::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn mongo_get_db_url() -> String {
    let mongo_first_handle_url_part = &CONFIG
        .mongo_params
        .mongo_url_parts
        .mongo_first_handle_url_part;
    let mongo_login = &CONFIG.mongo_params.mongo_authorization.mongo_login;
    let mongo_second_handle_url_part = &CONFIG
        .mongo_params
        .mongo_url_parts
        .mongo_second_handle_url_part;
    let mongo_password = &CONFIG.mongo_params.mongo_authorization.mongo_password;
    let mongo_third_handle_url_part = &CONFIG
        .mongo_params
        .mongo_url_parts
        .mongo_third_handle_url_part;
    let mongo_ip = &CONFIG.mongo_params.mongo_authorization.mongo_ip;
    let mongo_fourth_handle_url_part = &CONFIG
        .mongo_params
        .mongo_url_parts
        .mongo_fourth_handle_url_part;
    let mongo_port = &CONFIG.mongo_params.mongo_authorization.mongo_port;
    let mongo_fifth_handle_url_part = &CONFIG
        .mongo_params
        .mongo_url_parts
        .mongo_fifth_handle_url_part;
    let mongo_params = &CONFIG.mongo_params.mongo_authorization.mongo_params;
    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        mongo_first_handle_url_part,
        mongo_login,
        mongo_second_handle_url_part,
        mongo_password,
        mongo_third_handle_url_part,
        mongo_ip,
        mongo_fourth_handle_url_part,
        mongo_port,
        mongo_fifth_handle_url_part,
        mongo_params
    )
}
