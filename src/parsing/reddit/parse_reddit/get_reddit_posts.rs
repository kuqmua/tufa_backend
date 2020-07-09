extern crate roux;

use roux::Subreddit;

#[path = "../subreddit_rust_structs/reddit_post_data_wrapper.rs"]
mod reddit_post_data_wrapper;
//-> reddit_post_data_wrapper::RedditPostDataWrapper
pub fn get_reddit_posts(subreddits_vec: Vec<&str>) {
    let mut count_subreddits: u8 = 0;
    for subreddititer in subreddits_vec {
        count_subreddits += 1;
        let subreddit = Subreddit::new(subreddititer);
        println!("subreddit ------------------ {}", subreddititer);
        //let hot = subreddit.hot(1, None);
        let hot = subreddit.latest(1, None);
        let unwrapped_hot = &hot.unwrap();
        let data = &unwrapped_hot.data;
        let children = &data.children;
        let first_child = &children.first();
        match first_child {
            Some(_) => {
                let unwrapped_first_child = first_child.unwrap();
                let first_child_data = &unwrapped_first_child.data;
                let title = &first_child_data.title.clone();
                println!("{}", title);
            }
            None => println!("No first child"),
        }
    }
    println!("countSubreddits = {}", count_subreddits);
}
pub fn parse_reddit_post(subreddit_name: &str) {
    let subreddit = Subreddit::new(subreddit_name);
    let hot = subreddit.top(1, None);
    let unwrapped_hot = &hot.unwrap();
    let data = &unwrapped_hot.data;
    let children = &data.children;
    let first_child = children.first().unwrap();
    let first_child_data = &first_child.data;
    let title = first_child_data.title.clone();
    println!("{}", title);
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
    media_only,
    spoiler,
    is_original_content,
    quarantine,
    is_reddit_media_domain,
    is_meta,
    send_replies,
    is_self,
    allow_live_comments,
    saved,
    is_video,
    no_follow,
    contest_mode,
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
