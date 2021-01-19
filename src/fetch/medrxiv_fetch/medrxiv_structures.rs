extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlMedrxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlMedrxivParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlMedrxivParserStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
    pub date: String,
    pub publisher: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MedrxivPageStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<MedrxivPageStructItem>,
}

impl MedrxivPageStruct {
    pub fn new() -> Self {
        MedrxivPageStruct {
            // items: Vec::<MedrxivPageStructItem>::new(),
            items: vec![MedrxivPageStructItem::new(); 30],
            //vec![UsedRedditJsonStruct::new(); 25],
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MedrxivPageStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<String>,
    pub date: String,
    pub publisher: String,
}
impl MedrxivPageStructItem {
    pub fn new() -> Self {
        MedrxivPageStructItem {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<String>::new(),
            date: "".to_string(),
            publisher: "".to_string(),
        }
    }
}
