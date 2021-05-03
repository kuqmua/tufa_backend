#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsing {
    pub data: RedditStructForParsingVector,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsingVector {
    pub children: Vec<RedditStructForParsingVectorChild>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsingVectorChild {
    pub data: RedditStructForParsingVectorChildData,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsingVectorChildData {
    pub link: Option<String>,
    pub subreddit: String,
    pub selftext: String,
    pub id: String, // может понадобиться
    pub author: String,
    pub title: String,
    pub domain: String, //сайт хоста
    pub permalink: String,
    pub thumbnail: String, //todo
    pub created_utc: f64,  // время
    pub ups: f64,
    pub score: f64, //чем отличается score от ups
    pub num_comments: u64,
    pub over_18: bool,
    pub quarantine: bool,
    pub is_self: bool, //может понадобиться мб
    pub saved: bool,
    pub url: String,
}
