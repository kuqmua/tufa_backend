#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Children {
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data2 {
    pub subreddit: String,
    pub selftext: String,
    pub author_fullname: String,
    pub saved: bool,
    pub title: String,
    pub quarantine: bool,
    pub upvote_ratio: f64,
    pub ups: i64,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool, //maybe
    pub is_meta: bool,
    pub score: i64, //чем отличается score от ups
    pub is_self: bool,
    pub domain: String, //сайт хоста
    pub allow_live_comments: bool,
    pub selftext_html: Option<String>, // возможно юзлесс
    pub no_follow: bool,               // мб пригодится
    #[serde(rename = "over_18")]
    pub over18: bool,
    pub media_only: bool,
    pub spoiler: bool,
    pub id: String, // может понадобиться
    pub author: String,
    pub num_comments: i64,
    pub send_replies: bool,
    pub contest_mode: bool, // конкурс ли это - сразу фильтр такие
    pub permalink: String,
    pub url: String,
    pub post_hint: Option<String>, // тип поста
    pub subreddit_subscribers: i64,
    pub created_utc: f64,    // время
    pub num_crossposts: i64, // стоит ли чекать такой же пост на проверку в других разделах
    pub is_video: bool,
    pub url_overridden_by_dest: Option<String>, //url либо тут либо в url
    pub thumbnail: String,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
