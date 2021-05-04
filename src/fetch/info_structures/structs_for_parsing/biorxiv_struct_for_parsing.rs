#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<BiorxivStructForParsingItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivStructForParsingItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub date: String,
    pub creator: String,
    pub identifier: String,
    pub publisher: String,
    #[serde(rename = "publicationDate", default)]
    pub publication_date: String,
}
