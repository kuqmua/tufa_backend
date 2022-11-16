use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::mongo_integration::mongo_check_collection_is_not_empty::mongo_check_collections_is_not_empty;
use crate::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionsIsNotEmptyWrapperError;
use crate::mongo_integration::mongo_client_options_parse::mongo_client_options_parse;
use crate::mongo_integration::mongo_client_options_parse::MongoClientOptionsParseOriginError;
use crate::mongo_integration::mongo_client_with_options::mongo_client_with_options;
use crate::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginError;
use crate::mongo_integration::mongo_insert_many::mongo_insert_many;
use crate::mongo_integration::mongo_insert_many::MongoInsertManyOriginError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
)]
pub struct InitMongoWrapperError {
    source: InitMongoErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceFromTufaCommon)]
pub enum InitMongoErrorEnum {
    ClientOptionsParseWrapper(MongoClientOptionsParseOriginError),
    ClientWithOptionsWrapper(MongoClientWithOptionsOriginError),
    CollectionCountDocumentsOrIsNotEmptyWrapper(MongoCheckCollectionsIsNotEmptyWrapperError),
    InsertManyErrorWrapper(MongoInsertManyOriginError),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_mongo(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<(), Box<InitMongoWrapperError>> {
    todo!()
    // match mongo_client_options_parse(false).await {
    //     Err(e) => Err(Box::new(InitMongoWrapperError::init_error_with_possible_trace(
    //         InitMongoErrorEnum::ClientOptionsParse(*e),
    //         WhereWas {
    //             time: std::time::SystemTime::now()
    // .duration_since(std::time::UNIX_EPOCH)
    // .expect("cannot convert time to unix_epoch"),
    //             file: file!(),
    //             line: line!(),
    //             column: column!(),
    //         },
    //         &CONFIG.source_place_type,
    //         &GIT_INFO,
    //         should_trace,
    //     ))),
    //     Ok(client_options) => match mongo_client_with_options(client_options, false) {
    //         Err(e) => Err(Box::new(InitMongoWrapperError::init_error_with_possible_trace(
    //             InitMongoErrorEnum::ClientWithOptions(*e),
    //             WhereWas {
    //                 time: std::time::SystemTime::now()
    // .duration_since(std::time::UNIX_EPOCH)
    // .expect("cannot convert time to unix_epoch"),
    //                 file: file!(),
    //                 line: line!(),
    //                 column: column!(),
    //             },
    //             &CONFIG.source_place_type,
    //             &GIT_INFO,
    //             should_trace,
    //         ))),
    //         Ok(client) => {
    //             let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
    //             if let Err(error_vec_count_documents) = mongo_check_collections_is_not_empty(
    //                 providers_json_local_data_hashmap.clone(),
    //                 &db,
    //                 false,
    //             )
    //             .await
    //             {
    //                 return Err(Box::new(InitMongoWrapperError::init_error_with_possible_trace(
    //                     InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(
    //                         *error_vec_count_documents,
    //                     ),
    //                     WhereWas {
    //                         time: std::time::SystemTime::now()
    // .duration_since(std::time::UNIX_EPOCH)
    // .expect("cannot convert time to unix_epoch"),
    //                         file: file!(),
    //                         line: line!(),
    //                         column: column!(),
    //                     },
    //                     &CONFIG.source_place_type,
    //                     &GIT_INFO,
    //                     should_trace,
    //                 )));
    //             }
    //             if let Err(error_vec_insert_many) =
    //                 mongo_insert_many(providers_json_local_data_hashmap, db, false).await
    //             {
    //                 return Err(Box::new(InitMongoWrapperError::init_error_with_possible_trace(
    //                     InitMongoErrorEnum::InsertManyError(error_vec_insert_many),
    //                     WhereWas {
    //                         time: std::time::SystemTime::now()
    // .duration_since(std::time::UNIX_EPOCH)
    // .expect("cannot convert time to unix_epoch"),
    //                         file: file!(),
    //                         line: line!(),
    //                         column: column!(),
    //                     },
    //                     &CONFIG.source_place_type,
    //                     &GIT_INFO,
    //                     should_trace,
    //                 )));
    //             }
    //             // let error_vec_insert_many = join_all(providers_json_local_data_hashmap.iter().map(|(pk, data_vec)| async {
    //             //                         let docs: Vec<Document> = data_vec.iter().map(|data| doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }).collect();
    //             //                         (*pk, db.collection(&pk.get_db_tag()).insert_many(docs, None).await)
    //             //                     })).await
    //             //     .into_iter()
    //             //     .filter_map(|(pk, result)| {
    //             //         if let Err(e) = result {
    //             //             return Some((pk, e));
    //             //         }
    //             //         None
    //             //     })
    //             //     .collect::<HashMap<ProviderKind, Error>>();
    //             // if !error_vec_insert_many.is_empty() {
    //             //     return Err(Box::new(InitMongoWrapperError::init_error_with_possible_trace(
    //             //         InitMongoErrorEnum::InsertManyError(error_vec_insert_many),
    //             //         WhereWas {
    //             //             time: std::time::SystemTime::now()
    // .duration_since(std::time::UNIX_EPOCH)
    // .expect("cannot convert time to unix_epoch"),
    //             //             file: file!(),
    //             //             line: line!(),
    //             //             column: column!(),
    //             //         },
    //             //         &CONFIG.source_place_type,
    //             //         &GIT_INFO,
    //             //         should_trace,
    //             //     )));
    //             // }
    //             Ok(())
    //         }
    //     },
    // }
}
