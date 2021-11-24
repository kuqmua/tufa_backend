use crate::providers::provider_kind_enum::ProviderKind;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_item_handle(provider_kind: ProviderKind) -> Option<&'static str> {
        match provider_kind {
            ProviderKind::Arxiv => Some("</item>"),
            ProviderKind::Biorxiv => Some("</item>"),
            ProviderKind::Github => Some("</entry>"),
            ProviderKind::Habr => Some("</item>"),
            ProviderKind::Medrxiv => Some("</item>"),
            ProviderKind::Reddit => None,
            ProviderKind::Twitter => Some("</item>"),
        }
    }
}