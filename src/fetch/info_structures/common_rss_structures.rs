#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonRssPostStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<CommonRssPost>,
}

impl CommonRssPostStruct {
    pub fn new() -> Self {
        CommonRssPostStruct {
            items: Vec::<CommonRssPost>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonRssPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
}
impl CommonRssPost {
    pub fn new() -> Self {
        CommonRssPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creator: "".to_string(),
        }
    }
    pub fn initialize_with_params(
        title: String,
        link: String,
        description: String,
        creator: String,
    ) -> Self {
        CommonRssPost {
            title,
            link,
            description,
            creator,
        }
    }
}
