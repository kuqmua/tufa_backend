

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlBiorxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlBiorxivParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlBiorxivParserStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
    pub date: String,
    pub publisher: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPageStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<BiorxivPageStructItem>,
}

impl BiorxivPageStruct {
    pub fn new() -> Self {
        BiorxivPageStruct {
            // items: Vec::<BiorxivPageStructItem>::new(),
            items: vec![BiorxivPageStructItem::new(); 30],
            //vec![UsedRedditJsonStruct::new(); 25],
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPageStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<String>,
    pub date: String,
    pub publisher: String,
}
impl BiorxivPageStructItem {
    pub fn new() -> Self {
        BiorxivPageStructItem {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<String>::new(),
            date: "".to_string(),
            publisher: "".to_string(),
        }
    }
}