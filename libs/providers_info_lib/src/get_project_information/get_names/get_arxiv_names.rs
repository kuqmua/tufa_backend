// use mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;
use mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;

pub fn get_arxiv_names() -> Vec<String> {
    let mut arxiv_names: Vec<String> = Vec::new();
    let future_possible_vec_of_strings = mongo_get_provider_link_parts_as_bson_string(
  "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    "provider_links",
"arxiv_names".to_string(),
        "link_part",
        );
    match future_possible_vec_of_strings {
        Ok(vec_of_strings) => {
            println!("nice! {:#?}", vec_of_strings);
            arxiv_names = vec_of_strings;
        }
        Err(e) => {
            println!("F {:#?}", e);
        }
    }
    arxiv_names
}
