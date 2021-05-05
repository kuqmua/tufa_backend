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
    //meta
    pub provider_name: String,
    //meta
    //reddit specific

    //reddit specific
}
impl CommonRssPost {
    pub fn initialize_with_params(
        title: String,
        link: String,
        description: String,
        creator: String,
        //meta
        provider_name: String,
        //meta
        //reddit specific

        //reddit specific
    ) -> Self {
        CommonRssPost {
            title,
            link,
            description,
            creator,
            //meta
            provider_name,
            //meta
            //reddit specific

            //reddit specific
        }
    }
}
