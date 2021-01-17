#[path = "../parsing/reddit/parse_reddit/get_reddit_posts.rs"]
mod get_reddit_posts;
use get_reddit_posts::get_reddit_posts;

#[path = "../initialization/get_group_names/get_subreddits.rs"]
mod get_subredditss;
use get_subredditss::get_subreddits;
use std::time::Instant;

#[path = "../authorization/reddit/authorization_info.rs"]
mod authorization_info;
#[path = "../initialization/check_providers_status/can_i_reach_provider.rs"]
mod can_i_reach_provider;
#[path = "../authorization/reddit/reddit_authorization.rs"]
mod reddit_authorization;

pub fn reddit_part() {
    let f = can_i_reach_provider::can_i_reach_provider("https://www.reddit.com/".to_string());
    if f == true {
        let is_reddit_authorized = reddit_authorization::reddit_authorization(
            authorization_info::REDDIT_USER_AGENT,
            authorization_info::REDDIT_CLIENT_ID,
            authorization_info::REDDIT_CLIENT_SECRET,
            authorization_info::REDDIT_USERNAME,
            authorization_info::REDDIT_PASSWORD,
        );
        if is_reddit_authorized {
            println!("success reddit authorization");
            let time = Instant::now();
            let subreddits_names = get_subreddits();
            get_reddit_posts(subreddits_names);
            println!("reddit_part done in {} seconds", time.elapsed().as_secs());
        }
    }
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
