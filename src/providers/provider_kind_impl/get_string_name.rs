use crate::providers::provider_kind_enum::ProviderKind;

impl ProviderKind {
    pub fn get_string_name(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => "arxiv",
            ProviderKind::Biorxiv => "biorxiv",
            ProviderKind::Github => "github",
            ProviderKind::Habr => "habr",
            ProviderKind::Medrxiv => "medrxiv",
            ProviderKind::Reddit => "reddit",
            ProviderKind::Twitter => "twitter",
        }
    }
}
