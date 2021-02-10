extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;
use std::str;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlArxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlArxivParserPost>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlArxivParserPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPostStruct {
    pub items: Vec<ArxivPost>,
}
//count: usize
impl ArxivPostStruct {
    pub fn new() -> Self {
        ArxivPostStruct {
            items: Vec::<ArxivPost>::new(),
            // items: vec![ArxivPost::new(); count],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl ArxivPost {
    pub fn new() -> Self {
        ArxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
        }
    }
}
