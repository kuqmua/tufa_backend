#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlTwitterParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlTwitterParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlTwitterParserStructItem {
    pub title: String,
    pub creator: String,     //maybe creator == publisher
    pub description: String, //todo: parse inner links
    #[serde(rename = "pubDate", default)]
    pub publication_date: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TwitterPostStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<TwitterPost>,
}

impl TwitterPostStruct {
    pub fn new() -> Self {
        TwitterPostStruct {
            items: Vec::<TwitterPost>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TwitterPost {
    pub title: String,
    pub creator: String,
    pub description: String,
    pub publication_date: String,
    // pub guid: String,
    pub link: String,
}
impl TwitterPost {
    pub fn new() -> Self {
        TwitterPost {
            title: "".to_string(),
            creator: "".to_string(),
            description: "".to_string(),
            publication_date: "".to_string(),
            // guid: "".to_string(),
            link: "".to_string(),
        }
    }
}
