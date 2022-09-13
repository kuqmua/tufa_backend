use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::mongo::get_mongo_url::get_mongo_url;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use crate::traits::with_tracing::WithTracing;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::traits::get_source::GetSource;
use tufa_common::where_was::WhereWas;

#[derive(Debug, InitError, ImplGetSourceForParentErrorStruct)] //ImplGetWhereWasForErrorStruct,
pub struct InitMongoError {
    source: InitMongoErrorEnum,
    where_was: WhereWas,
}

impl crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for InitMongoError {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}

impl crate::traits::with_tracing::WithTracing<InitMongoErrorEnum> for InitMongoError {
    fn with_tracing(
        source: InitMongoErrorEnum,
        where_was: tufa_common::where_was::WhereWas,
    ) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.file_line_column(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    github_source_place =
                        where_was.github_file_line_column(&crate::helpers::git_info::GIT_INFO.data),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
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

impl tufa_common::traits::get_source::GetSource for InitMongoErrorEnum {
    fn get_source(&self) -> String {
        match self {
            InitMongoErrorEnum::ClientOptionsParse(e) => {
                match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable
                {
                    true => format!("{:#?}", e),
                    false => format!("{}", e),
                }
            }
            InitMongoErrorEnum::ClientWithOptions(e) => {
                match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable
                {
                    true => format!("{:#?}", e),
                    false => format!("{}", e),
                }
            }
            InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(e) => {
                match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable
                {
                    true => format!("{:#?}", e),
                    false => {
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
            InitMongoErrorEnum::InsertManyError(e) => {
                match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable
                {
                    true => format!("{:#?}", e),
                    false => {
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
    }
}

#[derive(Debug)]
pub enum CollectionCountDocumentsOrIsNotEmpty {
    CountDocuments(Error),
    IsNotEmpty(u64),
}

impl std::fmt::Display for CollectionCountDocumentsOrIsNotEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
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
    match ClientOptions::parse(&get_mongo_url()).await {
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(InitMongoError::with_tracing(
                    InitMongoErrorEnum::ClientOptionsParse(e),
                    where_was,
                ))),
                false => Err(Box::new(InitMongoError::new(
                    InitMongoErrorEnum::ClientOptionsParse(e),
                    where_was,
                ))),
            }
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                match should_trace {
                    true => Err(Box::new(InitMongoError::with_tracing(
                        InitMongoErrorEnum::ClientWithOptions(e),
                        where_was,
                    ))),
                    false => Err(Box::new(InitMongoError::new(
                        InitMongoErrorEnum::ClientWithOptions(e),
                        where_was,
                    ))),
                }
            }
            Ok(client) => {
                // let client_options = ClientOptions::parse(&mongo_get_db_url()).await?;
                // let client = Client::with_options(client_options)?;
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
                    let where_was = WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    };
                    match should_trace {
                        true => {
                            return Err(Box::new(InitMongoError::with_tracing(
                                InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(
                                    error_vec_count_documents,
                                ),
                                where_was,
                            )));
                        }
                        false => {
                            return Err(Box::new(InitMongoError::new(
                                InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(
                                    error_vec_count_documents,
                                ),
                                where_was,
                            )));
                        }
                    }
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
                    let where_was = WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    };
                    match should_trace {
                        true => {
                            return Err(Box::new(InitMongoError::with_tracing(
                                InitMongoErrorEnum::InsertManyError(error_vec_insert_many),
                                where_was,
                            )));
                        }
                        false => {
                            return Err(Box::new(InitMongoError::new(
                                InitMongoErrorEnum::InsertManyError(error_vec_insert_many),
                                where_was,
                            )));
                        }
                    }
                }
                Ok(())
            }
        },
    }
    //
}
