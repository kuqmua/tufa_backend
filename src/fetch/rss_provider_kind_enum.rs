use strum::EnumMessage;
#[derive(Clone, Debug, EnumMessage, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum ProviderKind {
    #[strum(message = "arxiv")]
    Arxiv,
    #[strum(message = "biorxiv")]
    Biorxiv,
    #[strum(message = "github")]
    Github,
    #[strum(message = "habr")]
    Habr,
    #[strum(message = "medrxiv")]
    Medrxiv,
    #[strum(message = "reddit")]
    Reddit,
    #[strum(message = "twitter")]
    Twitter,
}
