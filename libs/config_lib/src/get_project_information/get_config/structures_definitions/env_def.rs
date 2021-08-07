#[derive(Clone, Debug, serde_derive::Deserialize, PartialEq, serde_derive::Serialize)]
pub enum Env {
    Development,
    Testing,
    Production,
}
