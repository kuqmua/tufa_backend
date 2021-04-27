use crate::authorization::reddit::authorization_info;
use crate::authorization::reddit::reddit_authorization;
use crate::check_net::check_link::check_link;
// use crate::check_provider::can_i_reach_provider::reach_provider;
use crate::config::REDDIT_LINK;
use crate::fetch::reddit_fetch_wrapper::get_reddit_posts::get_reddit_posts;
use crate::get_group_names::get_subreddits::get_subreddits;

pub fn reddit_part() {
    if check_link(REDDIT_LINK).0 {
        println!("sss");
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
