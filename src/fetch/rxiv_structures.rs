#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlRxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlRxivParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlRxivParserStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
    // pub date: String,
    // pub publisher: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RxivPostStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<RxivPost>,
}

impl RxivPostStruct {
    pub fn new() -> Self {
        RxivPostStruct {
            items: Vec::<RxivPost>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub date: String,
    pub publisher: String,
}
impl RxivPost {
    pub fn new() -> Self {
        RxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            date: "".to_string(),
            publisher: "".to_string(),
        }
    }
}
