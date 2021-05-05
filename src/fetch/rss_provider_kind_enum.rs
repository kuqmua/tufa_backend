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
// impl fmt::Display for Foo {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }
