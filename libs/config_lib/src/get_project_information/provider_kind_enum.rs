#[derive(
    Clone, Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Hash, Copy,
)]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}
//todo: maybe use it like that Arxiv { string_name: "arxiv"} ?
