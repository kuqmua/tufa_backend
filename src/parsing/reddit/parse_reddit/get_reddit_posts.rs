extern crate roux;

use roux::Subreddit;

#[path = "../subreddit_rust_structs/reddit_post_data_wrapper.rs"]
mod reddit_post_data_wrapper;
use reddit_post_data_wrapper::RedditPostDataWrapper;
//use reddit_post_data_wrapper::SomeStruct;
//include!("../subreddit_rust_structs/reddit_post_data_wrapper.rs");
//-> reddit_post_data_wrapper::RedditPostDataWrapper
pub fn get_reddit_posts(subreddits_vec: Vec<&str>) -> Vec<RedditPostDataWrapper> {
    let mut count_subreddits: u8 = 0;
    let mut vec_reddit_post_data: Vec<RedditPostDataWrapper> = Vec::new();
    for subreddititer in subreddits_vec {
        count_subreddits += 1;
        let post = parse_reddit_post(subreddititer.to_string());

        vec_reddit_post_data.push(post);
    }
    println!("countSubreddits = {}", count_subreddits);
    vec_reddit_post_data
}

pub fn parse_reddit_post(subreddit_name: String) -> RedditPostDataWrapper {
    let subreddit = Subreddit::new(&subreddit_name);
    println!("subreddit ------------------ {}", subreddit_name);
    let latest = subreddit.latest(1, None);
    let unwrapped_latest = &latest.unwrap();
    let data = &unwrapped_latest.data;
    let children = &data.children;
    let first_child = &children.first();

    let mut redditwrapper = RedditPostDataWrapper::new();
    match first_child {
        Some(_) => {
            let unwrapped_first_child = first_child.unwrap();
            let first_child_data = &unwrapped_first_child.data;
            redditwrapper.subreddit = first_child_data.subreddit.clone();
            redditwrapper.selftext = first_child_data.selftext.clone();
            redditwrapper.id = first_child_data.id.clone();
            redditwrapper.author = first_child_data.author.clone();
            redditwrapper.title = first_child_data.title.clone();
            redditwrapper.domain = first_child_data.domain.clone();
            redditwrapper.permalink = first_child_data.permalink.clone();
            match &first_child_data.url {
                // The division was valid
                Some(x) => redditwrapper.url = Some(x.clone()),
                // The division was invalid
                None => redditwrapper.url = Some("None".to_string()),
            }
            redditwrapper.thumbnail = first_child_data.thumbnail.clone();
            redditwrapper.created_utc = first_child_data.created_utc.clone();
            redditwrapper.ups = first_child_data.ups.clone();
            redditwrapper.num_comments = first_child_data.num_comments.clone();
            redditwrapper.over_18 = first_child_data.over_18.clone();
            redditwrapper.score = first_child_data.score.clone();
            redditwrapper.quarantine = first_child_data.quarantine.clone();
            redditwrapper.is_self = first_child_data.is_self.clone();
            redditwrapper.saved = first_child_data.saved.clone();
        }
        None => println!("No first child"),
    }
    println!("redditwrapper ------------ {}", redditwrapper.subreddit);
    redditwrapper
}

//let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
/*
let RedditRedditPostDataWrapperVar = reddit_post_data_wrapper::RedditPostDataWrapper(
    subreddit: "oo",
    selftext,
    author_fullname,
    id,
    author,
    title,
    domain,
    permalink,
    url,
    thumbnail,
    thumbnail_width,
    thumbnail_height,
    selftext_html,
    url_overridden_by_dest,
    post_hint,
    upvote_ratio,
    ups,
    score,
    subreddit_subscribers,
    created_utc,
    num_crossposts,
    num_comments,
    over_18,
    quarantine,
    is_self,
    saved,
);
 */
/*
let mut count: u8 = 0;
        let mut authors = Vec::new();

        for child in data {
            count += 1;
            let author: String = child.data.author.clone();
            let title: String = child.data.title.clone();
            let ups: f64 = child.data.ups.clone();
            let over_18: bool = child.data.over_18.clone();
            println!("author = {}, selftext = {}, ups = {}", author, title, ups);
            authors.push(author);
        }
        println!("count = {}", count);
*/
