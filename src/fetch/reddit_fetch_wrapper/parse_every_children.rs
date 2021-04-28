use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStruct;
use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStructVectorChild;
use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::reddit_json_struct_vector::RedditJsonStruct;
use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::reddit_json_struct_vector::RedditJsonStructVector;

pub fn parse_every_children(
    u: &JsonRedditParserStruct,
    children: &Vec<JsonRedditParserStructVectorChild>,
) -> RedditJsonStructVector {
    let mut vec_of_children = RedditJsonStructVector::new();
    let mut count = 0;
    while count < children.len() {
        let mut child = RedditJsonStruct::new();
        child.link = u.data.children[count].data.link.clone();
        child.subreddit = u.data.children[count].data.subreddit.clone();
        //child.subreddit = &u.data.children[count].data.subreddit.clone();
        child.id = u.data.children[count].data.id.clone();
        child.author = u.data.children[count].data.author.clone();
        child.title = u.data.children[count].data.title.clone();
        child.domain = u.data.children[count].data.domain.clone();
        child.permalink = u.data.children[count].data.permalink.clone();
        child.thumbnail = u.data.children[count].data.thumbnail.clone();
        child.created_utc = u.data.children[count].data.created_utc.clone();
        child.ups = u.data.children[count].data.ups.clone();
        child.score = u.data.children[count].data.score.clone();
        child.num_comments = u.data.children[count].data.num_comments.clone();
        child.over_18 = u.data.children[count].data.over_18.clone();
        child.quarantine = u.data.children[count].data.quarantine.clone();
        child.is_self = u.data.children[count].data.is_self.clone();
        child.saved = u.data.children[count].data.saved.clone();
        vec_of_children.posts[count] = child;
        count += 1;
    }
    vec_of_children
}
