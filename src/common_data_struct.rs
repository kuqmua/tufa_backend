use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonDataStruct {
    pub common_data_struct_id: String,
    pub selftext: String,
    pub author: String,
    pub title: String,
    pub link_to_provider_post: String,
    pub created_time: f64,
    pub likes: f64,
    pub dislikes: f64,
    pub has_comments: bool,
    pub num_comments: u64,
    pub comments_url: Option<String>,
    pub has_avatar: bool,
    pub avatar_url: Option<String>,
    pub has_post_image: bool,
    pub post_image_url: Option<String>,
    pub has_multiple_post_images: bool,
    pub multiple_post_image_url: Option<String>,
}

impl Display for CommonDataStruct {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "url = {:?}\nsubreddit = {}\nselftext ={}\nid = {}\nauthor = {}\ntitle = {}\ndomain = {}\npermalink = {}\nups = {}\nscore = {}\ncreated_utc = {}\nnum_comments = {}\nover_18 = {}\nquarantine = {}\nis_self = {}\nsaved = {}\n",
            self.url,
            self.subreddit,
            self.selftext,
            self.common_data_struct_id,
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

impl CommonDataStruct {
    pub fn new() -> Self {
        CommonDataStruct {
            url: Some("".to_string()),
            subreddit: "".to_string(),
            selftext: "".to_string(),
            common_data_struct_id: "".to_string(),
            author: "".to_string(),
            title: "".to_string(),
            domain: "".to_string(),
            permalink: "".to_string(),
            thumbnail: "".to_string(),
            created_utc: 0.0,
            ups: 0.0,
            score: 0.0,
            num_comments: 0,
            over_18: false,
            quarantine: false,
            is_self: false,
            saved: false,
        }
    }
}
