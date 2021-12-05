use crate::config_mods::lazy_static_config::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn mongo_get_db_url() -> String {
    //maybe rename it into get mongo_url?
    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        &CONFIG.mongo_first_handle_url_part,
        &CONFIG.mongo_login,
        &CONFIG.mongo_second_handle_url_part,
        &CONFIG.mongo_password,
        &CONFIG.mongo_third_handle_url_part,
        &CONFIG.mongo_ip,
        &CONFIG.mongo_fourth_handle_url_part,
        &CONFIG.mongo_port,
        &CONFIG.mongo_fifth_handle_url_part,
        &CONFIG.mongo_params
    )
}
