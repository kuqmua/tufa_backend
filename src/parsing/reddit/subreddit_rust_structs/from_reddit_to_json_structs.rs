#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    pub kind: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    pub modhash: String,
    pub dist: i64,
    pub children: Vec<Children>,
    pub after: String,
    pub before: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Children {
    pub kind: String,
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data2 {
    pub url: Option<String>,
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
