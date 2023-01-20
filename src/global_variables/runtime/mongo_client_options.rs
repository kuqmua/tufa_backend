use crate::global_variables::runtime::config::CONFIG;
use mongodb::options::ClientOptions;
use once_cell::sync::Lazy;
use tufa_common::traits::get_mongo_url::GetMongoUrl;

pub static MONGO_CLIENT_OPTIONS: Lazy<ClientOptions> = Lazy::new(|| {
    futures::executor::block_on(ClientOptions::parse(CONFIG.get_mongo_url()))
        .expect("cannot construct mongo client options")
});
