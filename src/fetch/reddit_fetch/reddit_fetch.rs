use crate::authorization::reddit::authorization_info;
use crate::authorization::reddit::reddit_authorization;
use crate::check_provider::can_i_reach_provider::reach_provider;
use crate::config::REDDIT_URL;
use crate::fetch::reddit_fetch::get_reddit_posts::get_reddit_posts;
use crate::get_group_names::get_subreddits::get_subreddits;

pub fn reddit_part() {
    if reach_provider(REDDIT_URL.to_string()) {
        let is_reddit_authorized = reddit_authorization::reddit_authorization(
            authorization_info::REDDIT_USER_AGENT,
            authorization_info::REDDIT_CLIENT_ID,
            authorization_info::REDDIT_CLIENT_SECRET,
            authorization_info::REDDIT_USERNAME,
            authorization_info::REDDIT_PASSWORD,
        );
        if is_reddit_authorized {
            println!("success reddit authorization");
            let subreddits_names = get_subreddits();
            get_reddit_posts(subreddits_names); //возможно неподходящее название////тут есть возвращаемое значение let vec_of_vec_of_strings =
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
