use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplGetSourceForParentErrorStruct,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    InitErrorWithTracingForOriginalErrorStruct,
)]
pub struct InitMongoError {
    source: InitMongoErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug)]
pub enum InitMongoErrorEnum {
    ClientOptionsParse(mongodb::error::Error),
    ClientWithOptions(mongodb::error::Error),
    CollectionCountDocumentsOrIsNotEmpty(
        HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
    ),
    InsertManyError(HashMap<ProviderKind, Error>),
}

use impl_get_source_for_enum_with_method::ImplGetSourceForEnumWithMethod;

#[derive(ImplGetSourceForEnumWithMethod)] //Debug,
pub enum FFF {
    HandleOneUnnamed(One),
    HandleTwoUnnamed(Vec<Two>),
    HandleThreeUnnamed(HashMap<String, Three>),
    // HandleOneNamed {
    //     sfgd: One,
    // },
    // HandleTwoNamed {
    //     dsgdfg: Vec<Two>,
    // },
    // HandleThreeNamed {
    //     dfjgdfj: HashMap<String, Three>,
    // },
    // HandleMultiple {
    //     sdgfsd: One,
    //     sfdhsh: Vec<Two>,
    //     rerurt: HashMap<String, Three>,
    // },
}

#[derive(Debug)]
pub struct One {}

#[derive(Debug)]
pub struct Two {}

#[derive(Debug)]
pub struct Three {}

impl tufa_common::traits::get_source::GetSource for One {
    fn get_source(&self) -> String {
        String::from("one")
    }
}

impl tufa_common::traits::get_source::GetSource for Two {
    fn get_source(&self) -> String {
        String::from("two")
    }
}

impl tufa_common::traits::get_source::GetSource for Three {
    fn get_source(&self) -> String {
        String::from("three")
    }
}

impl tufa_common::traits::get_source::GetSource for InitMongoErrorEnum {
    fn get_source(&self) -> String {
        match self {
            InitMongoErrorEnum::ClientOptionsParse(e) => {
                format!("{}", e)
            }
            InitMongoErrorEnum::ClientWithOptions(e) => {
                format!("{}", e)
            }
            InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(e) => {
                //todo impl for get_source like deps hashmap iter fold support
                let mut formatted = e
                    .iter()
                    .map(|(pk, error)| format!("{} {},", pk, error))
                    .collect::<Vec<String>>()
                    .iter()
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(elem);
                        acc
                    });
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
            InitMongoErrorEnum::InsertManyError(e) => {
                let mut formatted = e
                    .iter()
                    .map(|(pk, error)| format!("{} {},", pk, error))
                    .collect::<Vec<String>>()
                    .iter()
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(elem);
                        acc
                    });
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
        }
    }
}

#[derive(Debug)]
pub enum CollectionCountDocumentsOrIsNotEmpty {
    CountDocuments(Error),
    IsNotEmpty(u64),
}

impl std::fmt::Display for CollectionCountDocumentsOrIsNotEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                CollectionCountDocumentsOrIsNotEmpty::CountDocuments(e) => write!(f, "{}", e),
                CollectionCountDocumentsOrIsNotEmpty::IsNotEmpty(e) => write!(f, "{}", e),
            },
        }
    }
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
) -> Result<(), Box<InitMongoError>> {
    match ClientOptions::parse(&CONFIG.get_mongo_url()).await {
        Err(e) => Err(Box::new(InitMongoError::init_error_with_possible_trace(
            InitMongoErrorEnum::ClientOptionsParse(e),
            WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
            &CONFIG.source_place_type,
            &GIT_INFO.data,
            should_trace,
        ))),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(InitMongoError::init_error_with_possible_trace(
                InitMongoErrorEnum::ClientWithOptions(e),
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ))),
            Ok(client) => {
                let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
                let error_vec_count_documents =
                    join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
                        (
                            *pk,
                            db.collection::<Document>(&pk.get_db_tag())
                                .count_documents(None, None)
                                .await,
                        )
                    }))
                    .await
                    .into_iter()
                    .filter_map(|(pk, result)| match result {
                        Err(e) => {
                            Some((pk, CollectionCountDocumentsOrIsNotEmpty::CountDocuments(e)))
                        }
                        Ok(documents_number) => {
                            if documents_number > 0 {
                                return Some((
                                    pk,
                                    CollectionCountDocumentsOrIsNotEmpty::IsNotEmpty(
                                        documents_number,
                                    ),
                                ));
                            }
                            None
                        }
                    })
                    .collect::<HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>>();
                if !error_vec_count_documents.is_empty() {
                    return Err(Box::new(InitMongoError::init_error_with_possible_trace(
                        InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(
                            error_vec_count_documents,
                        ),
                        WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    )));
                }
                drop(error_vec_count_documents);
                let error_vec_insert_many = join_all(providers_json_local_data_hashmap.iter().map(|(pk, data_vec)| async {
                                        let docs: Vec<Document> = data_vec.iter().map(|data| doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }).collect();
                                        (*pk, db.collection(&pk.get_db_tag()).insert_many(docs, None).await)
                                    })).await
                    .into_iter()
                    .filter_map(|(pk, result)| {
                        if let Err(e) = result {
                            return Some((pk, e));
                        }
                        None
                    })
                    .collect::<HashMap<ProviderKind, Error>>();
                if !error_vec_insert_many.is_empty() {
                    return Err(Box::new(InitMongoError::init_error_with_possible_trace(
                        InitMongoErrorEnum::InsertManyError(error_vec_insert_many),
                        WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    )));
                }
                Ok(())
            }
        },
    }
    //
}
