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
    pub items: Vec<BiorxivPost>,
}

impl BiorxivPageStruct {
    pub fn new() -> Self {
        BiorxivPageStruct {
            // items: Vec::<BiorxivPageStructItem>::new(),
            items: Vec::<BiorxivPost>::new(),
            // items: vec![BiorxivPageStructItem::new(); 30],
            //vec![UsedRedditJsonStruct::new(); 25],
        }
    }
}
// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// pub struct BiorxivPageStructItem {
//     pub title: String,
//     pub link: String,
//     pub description: String,
//     pub creators: Vec<String>,
//     pub date: String,
//     pub publisher: String,
// }

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<Creator>,
}

impl BiorxivPost {
    pub fn new() -> Self {
        BiorxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<Creator>::new(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Creator {
    pub name: String,
    pub link: String,
}

impl Creator {
    pub fn new() -> Self {
        Creator {
            name: "".to_string(),
            link: "".to_string(),
        }
    }
}
