pub fn get_arxiv_names() -> Vec<String> {
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let result_of_mongo_integration = mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string(
        "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    "testdatabase",
"testcollection",
    "link_part",
    );
    let mut arxiv_names: Vec<String> = Vec::new();
    match result_of_mongo_integration {
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
