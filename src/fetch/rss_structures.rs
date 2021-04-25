#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlRssParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlRssParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlRssParserStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RssPostStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<RssPost>,
}

impl RssPostStruct {
    pub fn new() -> Self {
        RssPostStruct {
            items: Vec::<RssPost>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RssPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
}
impl RssPost {
    pub fn new() -> Self {
        RssPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creator: "".to_string(),
        }
    }
}
