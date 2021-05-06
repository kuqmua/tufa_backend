use strum::EnumMessage;
#[derive(Clone, Debug, EnumMessage)]
pub enum ProviderKind {
    #[strum(message = "arxiv")]
    Arxiv,
    #[strum(message = "biorxiv")]
    Biorxiv,
    #[strum(message = "habr")]
    Habr,
    #[strum(message = "medrxiv")]
    Medrxiv,
    #[strum(message = "reddit")]
    Reddit,
    #[strum(message = "twitter")]
    Twitter,
}
