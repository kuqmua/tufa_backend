#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditJsonStructVector {
    pub posts: Vec<RedditJsonStruct>,
}

impl RedditJsonStructVector {
    pub fn new() -> Self {
        RedditJsonStructVector {
            // posts: vec![RedditJsonStruct::new(); 25], //default reddit api json children amount
            posts: Vec::<RedditJsonStruct>::new(),
        }
    }
}
//Vec::with_capacity(25)
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditJsonStruct {
    // pub link: Option<String>,
    // pub subreddit: String,
    //pub subreddit: &'static str,
    pub selftext: String,
    // pub id: String, // может понадобиться
    pub author: String,
    pub title: String,
    // pub domain: String, //сайт хоста
    // pub permalink: String,
    // pub thumbnail: String,
    // pub created_utc: f64, // время
    // pub ups: f64,
    // pub score: f64, //чем отличается score от ups
    // pub num_comments: u64,
    // pub over_18: bool,
    // pub quarantine: bool,
    // pub is_self: bool,
    // pub saved: bool,
    pub url: String,
}

impl RedditJsonStruct {
    pub fn new() -> Self {
        RedditJsonStruct {
            // link: Some("".to_string()),
            // subreddit: "".to_string(),
            //subreddit: "",
            selftext: "".to_string(),
            // id: "".to_string(), // может понадобиться
            author: "".to_string(),
            title: "".to_string(),
            // domain: "".to_string(), //сайт хоста
            // permalink: "".to_string(),
            // thumbnail: "".to_string(),
            // created_utc: 0.0, // время
            // ups: 0.0,
            // score: 0.0, //чем отличается score от ups
            // num_comments: 0,
            // over_18: false,
            // quarantine: false,
            // is_self: false,
            // saved: false,
            url: "".to_string(),
        }
    }
}
