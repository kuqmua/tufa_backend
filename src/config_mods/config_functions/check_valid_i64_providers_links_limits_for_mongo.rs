extern crate toml;

use crate::config_mods::config_struct::ConfigStruct;

impl ConfigStruct {
    fn check_valid_i64_providers_links_limits_for_mongo(config_handle: &ConfigStruct) -> bool {
        if config_handle.providers_links_limits.links_limit_for_arxiv <= 0 {
            return false;
        }
        if config_handle.providers_links_limits.links_limit_for_biorxiv <= 0 {
            return false;
        }
        if config_handle.providers_links_limits.links_limit_for_github <= 0 {
            return false;
        }
        if config_handle.providers_links_limits.links_limit_for_habr <= 0 {
            return false;
        }
        if config_handle.providers_links_limits.links_limit_for_medrxiv <= 0 {
            return false;
        }
        if config_handle.providers_links_limits.links_limit_for_reddit <= 0 {
            return false;
        }
        if config_handle.providers_links_limits.links_limit_for_twitter <= 0 {
            return false;
        }
        true
    }
}
