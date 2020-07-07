#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    pub subreddit: String,
    pub selftext: String,
    pub author_fullname: String,
    pub id: String, // может понадобиться
    pub author: String,
    pub title: String,
    pub domain: String, //сайт хоста
    pub permalink: String,
    pub url: String,
    pub thumbnail: String,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
    pub selftext_html: Option<String>,          // возможно юзлесс
    pub url_overridden_by_dest: Option<String>, //url либо тут либо в url
    pub post_hint: Option<String>,              // тип поста
    pub upvote_ratio: f64,
    pub ups: i64,
    pub score: i64, //чем отличается score от ups
    pub subreddit_subscribers: i64,
    pub created_utc: f64,    // время
    pub num_crossposts: i64, // стоит ли чекать такой же пост на проверку в других разделах
    pub num_comments: i64,
    #[serde(rename = "over_18")]
    pub over18: bool,
    pub media_only: bool,
    pub spoiler: bool,
    pub is_original_content: bool,
    pub quarantine: bool,
    pub is_reddit_media_domain: bool, //maybe
    pub is_meta: bool,
    pub send_replies: bool,
    pub is_self: bool,
    pub allow_live_comments: bool,
    pub saved: bool,
    pub is_video: bool,
    pub no_follow: bool,    // мб пригодится
    pub contest_mode: bool, // конкурс ли это - сразу фильтр такие
}
