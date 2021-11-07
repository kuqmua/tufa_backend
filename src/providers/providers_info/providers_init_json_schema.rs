#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ProvidersInitJsonSchema {
    pub data: Vec<String>,
}
