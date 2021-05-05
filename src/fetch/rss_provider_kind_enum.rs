#[derive(Clone, Debug)]
pub enum ProviderKind {
    Biorxiv,
    Arxiv,
    Medrxiv,
    Twitter,
    Reddit,
    Habr,
}
// impl fmt::Display for Foo {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }
