extern crate toml;

use crate::config_mods::config_struct::ConfigStruct;

impl ConfigStruct {
    pub fn check_valid_i64_providers_links_limits_for_mongo(config_handle: &ConfigStruct) -> bool {
        if config_handle.links_limit_arxiv <= 0 {
            return false;
        }
        if config_handle.links_limit_biorxiv <= 0 {
            return false;
        }
        if config_handle.links_limit_github <= 0 {
            return false;
        }
        if config_handle.links_limit_habr <= 0 {
            return false;
        }
        if config_handle.links_limit_medrxiv <= 0 {
            return false;
        }
        if config_handle.links_limit_reddit <= 0 {
            return false;
        }
        if config_handle.links_limit_twitter <= 0 {
            return false;
        }
        true
    }
}
