use crate::constants::project_constants::TWITTER_LINK_FIRST_PART;
use crate::constants::project_constants::TWITTER_LINK_SECOND_PART;
use crate::constants::project_constants::TWITTER_LINK_THIRD_PART;

use crate::providers::providers_info::get_twitter_providers_names::get_twitter_providers_names;

pub fn generate_twitter_links(twitter_subs_names: Vec<String>) -> Vec<String> {
    //example https://nitter.pussthecat.org/Tom_McGurl/rss
    //todo: remove vec and remane only string
    let twitter_provider_names = get_twitter_providers_names();
    //todo: move this assertion into config new function
    assert!(
        !twitter_provider_names.is_empty(),
        "twitter_provider_names is empty!!!"
    );
    assert!(
        !twitter_subs_names.is_empty(),
        "twitter_subs_names is empty!"
    );
    let twitter_subs_names_length = twitter_subs_names.len();
    let mut twitter_sections_links: Vec<String> = Vec::with_capacity(twitter_subs_names_length);
    if twitter_subs_names_length > twitter_provider_names.len() {
        let how_many_twitter_links_on_diff_provider =
            twitter_subs_names_length / twitter_provider_names.len();
        let twitter_provider_names_length = twitter_provider_names.len();

        let mut twitter_provider_name_index = 0;

        for (twitter_sub_name_index, sub_name) in twitter_subs_names.into_iter().enumerate() {
            let sub_link: String = format!(
                "{}{}{}{}{}",
                TWITTER_LINK_FIRST_PART,
                twitter_provider_names[twitter_provider_name_index],
                TWITTER_LINK_SECOND_PART,
                sub_name,
                TWITTER_LINK_THIRD_PART
            );
            twitter_sections_links.push(sub_link);
            if twitter_sub_name_index != 0
                && twitter_sub_name_index % how_many_twitter_links_on_diff_provider == 0
                && (twitter_provider_names_length - 1) > twitter_provider_name_index
            {
                twitter_provider_name_index += 1;
            }
        }
    } else {
        let twitter_provider_names_splited = &twitter_provider_names[0..twitter_subs_names_length];
        for (index, sub_name) in twitter_subs_names.into_iter().enumerate() {
            let sub_link: String = format!(
                "{}{}{}{}{}",
                TWITTER_LINK_FIRST_PART,
                twitter_provider_names_splited[index],
                TWITTER_LINK_SECOND_PART,
                sub_name,
                TWITTER_LINK_THIRD_PART
            );
            twitter_sections_links.push(sub_link);
        }
    }
    assert!(
        !twitter_sections_links.is_empty(),
        "twitter_sections_links is empty in generate_twitter_hashmap_links()!!!"
    );
    twitter_sections_links //maybe change structure for memory effective reasons
}
