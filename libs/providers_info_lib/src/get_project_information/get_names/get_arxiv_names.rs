use mongo_integration::mongo_drop_collection_non_check::mongo_drop_collection_non_check;
use mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;

pub fn get_arxiv_names() -> Vec<String> {
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let future_possible_vec_of_strings = mongo_get_provider_link_parts_as_bson_string(
        "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    "testdatabase",
"testcollection",
    "link_part",
    );
    let mut arxiv_names: Vec<String> = Vec::new();
    match future_possible_vec_of_strings {
        Ok(vec_of_strings) => {
            println!("nice! {:#?}", vec_of_strings);
            arxiv_names = vec_of_strings;
        }
        Err(e) => {
            println!("F {:#?}", e);
        }
    }
    let future_possible_drop_collection = mongo_drop_collection_non_check(
        "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    "testdatabase",
"testcollection",
    );
    match future_possible_drop_collection {
        Ok(result_flag) => {
            if result_flag {
                println!("drop done!");
            } else {
                println!("drop fail with flag");
            }
        }
        Err(e) => {
            println!("drop fail with error {:#?}", e);
        }
    }
    arxiv_names
}
