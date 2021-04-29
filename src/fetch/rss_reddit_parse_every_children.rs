use crate::fetch::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStruct;
use crate::fetch::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStructVectorChild;
use crate::fetch::reddit_json_structs::reddit_json_struct_vector::RedditJsonStruct;
use crate::fetch::reddit_json_structs::reddit_json_struct_vector::RedditJsonStructVector;

pub fn rss_reddit_parse_every_children(
    json_reddit_parser_struct: &JsonRedditParserStruct,
    json_reddit_parser_struct_vector_children: &[JsonRedditParserStructVectorChild],
) -> RedditJsonStructVector {
    let mut vec_of_children = RedditJsonStructVector::new();
    for (index, _) in json_reddit_parser_struct_vector_children.iter().enumerate() {
        let mut reddit_json_struct = RedditJsonStruct::new();
        reddit_json_struct.selftext = json_reddit_parser_struct.data.children[index]
            .data
            .selftext
            .clone();
        reddit_json_struct.author = json_reddit_parser_struct.data.children[index]
            .data
            .author
            .clone();
        reddit_json_struct.title = json_reddit_parser_struct.data.children[index]
            .data
            .title
            .clone();
        reddit_json_struct.url = json_reddit_parser_struct.data.children[index]
            .data
            .url
            .clone();
        vec_of_children.posts.push(reddit_json_struct);
    }
    vec_of_children
}
