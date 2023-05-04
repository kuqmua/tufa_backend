// #[derive(Debug)]
// pub struct RemoveDirError {
//     pub error: std::io::Error,
// }

// #[derive(Debug)]
// pub enum CleanLogsDirError {
//     PathIsNotDir { path: String },
//     CannotRemoveDir { error: RemoveDirError },
// }
// impl From<String> for CleanLogsDirError {
//     fn from(e: String) -> Self {
//         CleanLogsDirError::PathIsNotDir { path: e }
//     }
// }
// impl From<std::io::Error> for CleanLogsDirError {
//     fn from(e: std::io::Error) -> Self {
//         CleanLogsDirError::CannotRemoveDir {
//             error: RemoveDirError { error: e },
//         }
//     }
// }

// #[derive(
//     provider_kind_from_config::ProviderKindFromConfig,
//     enum_extension::EnumExtension,
//     strum_macros::EnumIter,
//     Clone,
//     Debug,
//     serde_derive::Serialize,
//     serde_derive::Deserialize,
//     PartialEq,
//     Eq,
//     Hash,
//     Copy,
//     strum_macros::Display,
// )]
// #[strum(serialize_all = "snake_case")]
// pub enum ProviderKind {
//     Arxiv,
//     Biorxiv,
//     Github,
//     Habr,
//     Medrxiv,
//     Reddit,
//     Twitter,
// }
