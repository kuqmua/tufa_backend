#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlHabrParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlHabrParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlHabrParserStructItem {
    pub title: String,
    pub guid: String,
    pub link: String,
    pub description: String,
    #[serde(rename = "pubDate", default)]
    pub pub_date: String,
    pub creator: String,
    pub category: Vec<String>,
}
