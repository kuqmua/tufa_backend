use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::reddit_json_struct_vector::RedditJsonStructVector;

pub fn push_names_into_two_layer_result_vec(
    subreddits_vec: &Vec<&str>,
) -> Vec<RedditJsonStructVector> {
    let mut subreddit_names_vec: Vec<RedditJsonStructVector> =
        Vec::with_capacity(subreddits_vec.len());
    let mut count = 0;
    while count < subreddits_vec.len() {
        subreddit_names_vec.push(RedditJsonStructVector::new());
        count += 1;
    }
    subreddit_names_vec
}
