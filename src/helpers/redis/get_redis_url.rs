use crate::config_mods::lazy_static_config::CONFIG;

pub fn get_redis_url() -> String {
    format!("redis://{}:{}", CONFIG.redis_ip, CONFIG.redis_port)
}
