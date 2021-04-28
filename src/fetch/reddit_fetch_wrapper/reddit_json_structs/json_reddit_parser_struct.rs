#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JsonRedditParserStruct {
    pub data: JsonRedditParserStructVector,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JsonRedditParserStructVector {
    pub children: Vec<JsonRedditParserStructVectorChild>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JsonRedditParserStructVectorChild {
    pub data: JsonRedditParserStructVectorChildData,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JsonRedditParserStructVectorChildData {
    pub link: Option<String>,
    pub subreddit: String,
    pub selftext: String,
    pub id: String, // может понадобиться
    pub author: String,
    pub title: String,
    pub domain: String, //сайт хоста
    pub permalink: String,
    pub thumbnail: String,
    pub created_utc: f64, // время
    pub ups: f64,
    pub score: f64, //чем отличается score от ups
    pub num_comments: u64,
    pub over_18: bool,
    pub quarantine: bool,
    pub is_self: bool,
    pub saved: bool,
}
