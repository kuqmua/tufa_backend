use std::collections::HashMap;
use std::fmt;

use futures::future::join_all;

use mongodb::error::Error;

use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait,
};

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct InitMongoError {
    /// check link status code error `{0}`
    pub source: Box<InitMongoErrorEnum>,
}

//its needed for ImplDisplayDerive to work coz i dont implement some logic inside for HashMap type
type CountDocumentsErrorHashMap = HashMap<ProviderKind, CountDocumentsError>;
type InsertManyErrorHashMap = HashMap<ProviderKind, Error>;

#[derive(Debug, ImplFromForUpperStruct)]
pub enum InitMongoErrorEnum {
    Client(Error),
    CountDocumentsError(CountDocumentsErrorHashMap),
    InsertManyError(InsertManyErrorHashMap),
}

#[derive(Debug)]
pub enum CountDocumentsError {
    CollectionOperation(Error),
    CollectionIsNotEmpty(u64),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_mongo(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), InitMongoError> {
    let client_options = ClientOptions::parse(&mongo_get_db_url()).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&CONFIG.mongo_providers_logs_db_name); //<- todo this is incorrect name
    let vec_of_futures_count_documents = providers_json_local_data_hashmap.keys().map(|pk| async {
        (
            *pk,
            db.collection::<Document>(&pk.get_db_tag())
                .count_documents(None, None)
                .await,
        )
    });
    let mut error_vec_count_documents: HashMap<ProviderKind, CountDocumentsError> = HashMap::new();
    for (pk, result) in join_all(vec_of_futures_count_documents).await {
        match result {
            //todo filter
            Err(e) => {
                error_vec_count_documents.insert(pk, CountDocumentsError::CollectionOperation(e));
            }
            Ok(documents_number) => {
                if documents_number > 0 {
                    error_vec_count_documents.insert(
                        pk,
                        CountDocumentsError::CollectionIsNotEmpty(documents_number),
                    );
                }
            }
        }
    }
    if !error_vec_count_documents.is_empty() {
        return Err(InitMongoError {
            source: Box::new(InitMongoErrorEnum::CountDocumentsError(
                error_vec_count_documents,
            )),
        });
    }
    drop(error_vec_count_documents);
    let vec_of_futures_insert_many = providers_json_local_data_hashmap.iter().map(|(pk, data_vec)| async {
                            let collection = db.collection(&pk.get_db_tag());
                            let docs: Vec<Document> = data_vec.iter().map(|data| doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }).collect();
                            (*pk, collection.insert_many(docs, None).await)
                        });
    let mut error_vec_insert_many: HashMap<ProviderKind, Error> = HashMap::new();
    for (pk, result) in join_all(vec_of_futures_insert_many).await {
        if let Err(e) = result {
            error_vec_insert_many.insert(pk, e);
        }
    }
    if !error_vec_insert_many.is_empty() {
        return Err(InitMongoError {
            source: Box::new(InitMongoErrorEnum::InsertManyError(error_vec_insert_many)),
        });
    }
    drop(error_vec_insert_many);
    Ok(())
}
