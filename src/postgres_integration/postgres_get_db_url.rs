use crate::config_mods::lazy_static_config::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_get_db_url() -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        CONFIG.postgres_first_handle_url_part,
        CONFIG.postgres_login,
        CONFIG.postgres_second_handle_url_part,
        CONFIG.postgres_password,
        CONFIG.postgres_third_handle_url_part,
        CONFIG.postgres_ip,
        CONFIG.postgres_fourth_handle_url_part,
        CONFIG.postgres_port,
        CONFIG.postgres_fifth_handle_url_part,
        CONFIG.postgres_db
    )
}
