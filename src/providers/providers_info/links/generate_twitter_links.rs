pub fn generate_twitter_links(
    twitter_provider_names: Vec<String>,
    twitter_subs_names: Vec<String>,
) -> Vec<String> {
    //example https://nitter.pussthecat.org/Tom_McGurl/rss
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
                "https://{}/{}/rss",
                twitter_provider_names[twitter_provider_name_index], sub_name,
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
                "https://{}/{}/rss",
                twitter_provider_names_splited[index], sub_name,
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
