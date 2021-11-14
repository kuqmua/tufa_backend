use crate::constants::project_constants::TWITTER_LINK_FIRST_PART;
use crate::constants::project_constants::TWITTER_LINK_SECOND_PART;
use crate::constants::project_constants::TWITTER_LINK_THIRD_PART;

use crate::providers::providers_info::get_twitter_provider_name::get_twitter_provider_name;

pub fn generate_twitter_links(twitter_subs_names: Vec<String>) -> Vec<String> {
    //example https://nitter.pussthecat.org/Tom_McGurl/rss
    let mut twitter_sections_links: Vec<String> = Vec::with_capacity(twitter_subs_names.len());
    for sub_name in twitter_subs_names {
        let sub_link: String = format!(
            "{}{}{}{}{}",
            TWITTER_LINK_FIRST_PART,
            get_twitter_provider_name(),
            TWITTER_LINK_SECOND_PART,
            sub_name,
            TWITTER_LINK_THIRD_PART
        );
        twitter_sections_links.push(sub_link);
    }
    twitter_sections_links //maybe change structure for memory effective reasons
}
