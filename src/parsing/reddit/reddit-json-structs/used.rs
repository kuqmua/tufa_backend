use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct UsedRedditJsonStruct {
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

impl Display for UsedRedditJsonStruct {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "url = {:?}\nsubreddit = {}\nselftext ={}\nid = {}\nauthor = {}\ntitle = {}\ndomain = {}\npermalink = {}\nups = {}\nscore = {}\ncreated_utc = {}\nnum_comments = {}\nover_18 = {}\nquarantine = {}\nis_self = {}\nsaved = {}\n",
            self.url,
            self.subreddit,
            self.selftext,
            self.id,
            self.author,
            self.title,
            self.domain,
            self.permalink,
            self.ups,
            self.score,
            self.created_utc,
            self.num_comments,
            self.over_18,
            self.quarantine,
            self.is_self,
            self.saved,
        )
    }
}

impl UsedRedditJsonStruct {
    pub fn new() -> Self {
        UsedRedditJsonStruct {
            url: Some("url".to_string()),
            subreddit: "subreddit".to_string(),
            selftext: "selftext".to_string(),
            id: "id".to_string(), // может понадобиться
            author: "id".to_string(),
            title: "title".to_string(),
            domain: "domain".to_string(), //сайт хоста
            permalink: "permalink".to_string(),
            thumbnail: "thumbnail".to_string(),
            created_utc: 0.64, // время
            ups: 0.0,
            score: 0.0, //чем отличается score от ups
            num_comments: 64,
            over_18: false,
            quarantine: false,
            is_self: false,
            saved: false,
        }
    }
}
