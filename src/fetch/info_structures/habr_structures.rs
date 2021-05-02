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
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HabrPostStruct {
    // #[serde(rename = "item", default)]
    pub items: Vec<HabrPost>,
}

impl HabrPostStruct {
    pub fn new() -> Self {
        HabrPostStruct {
            items: Vec::<HabrPost>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HabrPost {
    pub title: String,
    pub guid: String,
    pub link: String,
    pub description: String,
    pub pubdate: String,
    pub creator: String,
}
impl HabrPost {
    pub fn new() -> Self {
        HabrPost {
            title: "".to_string(),
            guid: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            pubdate: "".to_string(),
            creator: "".to_string(),
        }
    }
    pub fn initialize_new(
        title: String,
        guid: String,
        link: String,
        description: String,
        pubdate: String,
        creator: String,
    ) -> Self {
        HabrPost {
            title,
            guid,
            link,
            description,
            pubdate,
            creator,
        }
    }
}
