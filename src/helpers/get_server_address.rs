use crate::config_mods::lazy_static_config::CONFIG;

pub fn get_server_address() -> String {
    format!("{}:{}", CONFIG.server_ip, CONFIG.server_port)
}
