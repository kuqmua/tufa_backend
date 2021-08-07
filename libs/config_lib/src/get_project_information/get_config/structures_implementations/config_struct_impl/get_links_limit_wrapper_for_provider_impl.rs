use crate::get_project_information::get_config::structures_definitions::config_struct_def::ConfigStruct;
use crate::get_project_information::provider_kind_enum::ProviderKind;

impl ConfigStruct {
    pub fn get_links_limit_wrapper_for_provider(self, provider_kind: &ProviderKind) -> i64 {
        match provider_kind {
            ProviderKind::Arxiv => self.providers_links_limits.links_limit_for_arxiv,
            ProviderKind::Biorxiv => self.providers_links_limits.links_limit_for_biorxiv,
            ProviderKind::Github => self.providers_links_limits.links_limit_for_github,
            ProviderKind::Habr => self.providers_links_limits.links_limit_for_habr,
            ProviderKind::Medrxiv => self.providers_links_limits.links_limit_for_medrxiv,
            ProviderKind::Reddit => self.providers_links_limits.links_limit_for_reddit,
            ProviderKind::Twitter => self.providers_links_limits.links_limit_for_twitter,
        }
    }
}
