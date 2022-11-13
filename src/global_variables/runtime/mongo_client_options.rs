use crate::global_variables::runtime::config::CONFIG;
use mongodb::options::ClientOptions;
use once_cell::sync::Lazy;

pub static MONGO_CLIENT_OPTIONS: Lazy<ClientOptions> = Lazy::new(|| {
    futures::executor::block_on(ClientOptions::parse({
        use tufa_common::config_mods::traits::get_mongo_url_trait::GetMongoUrl;
        CONFIG.get_mongo_url()
    }))
    .expect("cannot construct mongo client options")
});
