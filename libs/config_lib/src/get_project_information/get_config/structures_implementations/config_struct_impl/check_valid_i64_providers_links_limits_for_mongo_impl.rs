use crate::get_project_information::get_config::structures_definitions::config_struct_def::ConfigStruct;

impl ConfigStruct {
    //todo: find out why cannot write this path crate::get_project_information::get_config::structures_implementations::config_struct_impl::wrap_config_checks_impl
    pub(in crate::get_project_information::get_config::structures_implementations::config_struct_impl) fn check_valid_i64_providers_links_limits_for_mongo(
        config_handle: &ConfigStruct,
    ) -> bool {
        let mut checker = true;
        if config_handle.providers_links_limits.links_limit_for_arxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_biorxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_github <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_habr <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_medrxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_reddit <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_twitter <= 0 {
            checker = false;
        }
        checker
    }
}
