use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditPostDataWrapper {
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
    pub upvote_ratio: f64,
    pub created_utc: f64, // время
    pub ups: i64,
    pub score: i64, //чем отличается score от ups
    pub subreddit_subscribers: i64,
    pub num_crossposts: i64, // стоит ли чекать такой же пост на проверку в других разделах
    pub num_comments: i64,
    pub over_18: bool,
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

impl Display for RedditPostDataWrapper {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} ", 
        self.subreddit,
            self.selftext,
            self.author_fullname,
            self.id,
            self.author,
            self.title,
            self.domain,
            self.permalink,
            self.url,
            self.upvote_ratio,
            self.ups,
            self.score,
            self.subreddit_subscribers,
            self.created_utc,
            self.num_crossposts,

            self.num_comments,

            self.over_18,

            self.media_only,
            self.spoiler,
            self.is_original_content,
            self.quarantine,
            self.is_reddit_media_domain,
            self.is_meta,
            self.send_replies,
            self.is_self,
            self.allow_live_comments,
            self.saved,
            self.is_video,
            self.no_follow,
            self.contest_mode,
    )
    }
}
/*
impl Display for RedditPostDataWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} )",
            self.subreddit,
            self.selftext,
            self.author_fullname,
            self.id,
            self.author,
            self.title,
            self.domain,
            self.permalink,
            self.url,
            self.thumbnail,
            self.thumbnail_width,
            self.thumbnail_height,
            self.selftext_html,
            self.url_overridden_by_dest,
            self.post_hint,
            self.upvote_ratio,
            self.ups,
            self.score,
            self.subreddit_subscribers,
            self.created_utc,
            self.num_crossposts,
            self.num_comments,
            self.over_18,
            self.media_only,
            self.spoiler,
            self.is_original_content,
            self.quarantine,
            self.is_reddit_media_domain,
            self.is_meta,
            self.send_replies,
            self.is_self,
            self.allow_live_comments,
            self.saved,
            self.is_video,
            self.no_follow,
            self.contest_mode,
        )
    }
}
*/
/*
pub thumbnail_width: Option<i64>, // пока выдернем Option
    pub thumbnail_height: Option<i64>,// пока выдернем Option
    pub selftext_html: Option<String>,          // возможно юзлесс
    pub url_overridden_by_dest: Option<String>, //url либо тут либо в url
    pub post_hint: Option<String>,              // тип поста
*/
/*
self.thumbnail,
            self.thumbnail_width,
            self.thumbnail_height,
            self.selftext_html,
            self.url_overridden_by_dest,
            self.post_hint,
*/
