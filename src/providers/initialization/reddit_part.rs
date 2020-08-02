#[path = "../parsing/reddit/parse_reddit/get_reddit_posts.rs"]
mod get_reddit_posts;
use get_reddit_posts::get_reddit_posts;

#[path = "./get_group_names/get_subreddits.rs"]
mod get_subreddits;
use get_subreddits::get_subreddits;
use std::time::Instant;

pub fn reddit_part() {
    let time = Instant::now();
    let subreddits_names = get_subreddits();
    get_reddit_posts(subreddits_names);
    println!("reddit_part done in {} seconds", time.elapsed().as_secs());
}
// println!(
//     "{:?}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
//     vec[0].posts[0].url,
//     vec[0].posts[0].subreddit,
//     vec[0].posts[0].selftext,
//     vec[0].posts[0].id,
//     vec[0].posts[0].author,
//     vec[0].posts[0].title,
//     vec[0].posts[0].domain,
//     vec[0].posts[0].permalink,
//     vec[0].posts[0].thumbnail,
//     vec[0].posts[0].created_utc,
//     vec[0].posts[0].ups,
//     vec[0].posts[0].score,
//     vec[0].posts[0].num_comments,
//     vec[0].posts[0].over_18,
//     vec[0].posts[0].quarantine,
//     vec[0].posts[0].is_self,
//     vec[0].posts[0].saved,
// );
