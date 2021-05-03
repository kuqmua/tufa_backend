#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TwitterStructForParsing {
    #[serde(rename = "item", default)]
    pub items: Vec<TwitterStructForParsingItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TwitterStructForParsingItem {
    pub title: String,
    pub link: String,
    pub description: String,
    #[serde(rename = "pubDate", default)]
    pub pub_date: String,
    pub guid: String,
    pub creator: String,
}
