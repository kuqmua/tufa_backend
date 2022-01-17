use std::fmt;

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait,
};

use crate::{
    mongo_integration::mongo_get_db_url::mongo_get_db_url,
    prints::{print_colorful_message::print_colorful_message, print_type_enum::PrintType},
    providers::provider_kind_enum::ProviderKind,
};

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse(mongodb::error::Error),
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with  third party crate
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_provider_link_parts(
        pk: ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        //todo maybe option vec string
        //declare db name. there is no create db method in mongo
        let db = Client::with_options(ClientOptions::parse(mongo_get_db_url()).await?)?
            .database(&CONFIG.mongo_providers_logs_db_name);
        let collection = db.collection::<Document>(&pk.get_mongo_log_collection_name());
        let documents_number = collection.count_documents(None, None).await?;
        if documents_number > 0 {
            //rewrite as PrintType::Info or something
            print_colorful_message(
                None,
                PrintType::Success,
                file!().to_string(),
                line!().to_string(),
                format!("collection.count_documents {}", documents_number),
            );
            let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>;
            if CONFIG.is_links_limit_enabled_providers {
                if CONFIG.is_links_limit_providers_enabled {
                    if CONFIG.is_mongo_link_parts_randomize_order_enabled {
                        option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                            Some(doc! { "$sample" : {"size": CONFIG.links_limit_providers }});
                    } else {
                        option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                            Some(doc! { "$limit" :  CONFIG.links_limit_providers });
                    }
                } else {
                    option_aggregation_stage_1_get_docs_in_random_order_with_limit =
                        pk.get_mongo_doc_randomization_aggregation();
                }
            } else {
                option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
            }
            // let aggregation_stage_1_get_docs_in_random_order_with_limit =
            //     doc! { "$sample" : {"size": 5 }};
            // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
            let vec_of_strings = mongo_get_documents_as_string_vector(
                collection,
                &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                option_aggregation_stage_1_get_docs_in_random_order_with_limit,
            )
            .await?;
            return Ok(vec_of_strings);
        }
        Ok(Vec::new())
    }
}
