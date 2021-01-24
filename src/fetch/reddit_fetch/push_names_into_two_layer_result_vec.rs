use super::reddit_json_structs::used::VecOfUsedRedditJsonStruct;

pub fn push_names_into_two_layer_result_vec(
    subreddits_vec: &Vec<&str>,
) -> Vec<VecOfUsedRedditJsonStruct> {
    let mut subreddit_names_vec: Vec<VecOfUsedRedditJsonStruct> =
        Vec::with_capacity(subreddits_vec.len());
    let mut count = 0;
    while count < subreddits_vec.len() {
        subreddit_names_vec.push(VecOfUsedRedditJsonStruct::new());
        count += 1;
    }
    subreddit_names_vec
}
