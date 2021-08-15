#[derive(Clone, Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}
