use super::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorEnum;
use crate::global_variables::runtime::config::CONFIG;
use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::config_mods::traits::get_mongo_url_trait::GetMongoUrl;

#[derive(Debug)]
pub struct MongoGetProvidersLinkPartsError {
    pub source: Box<MongoGetProvidersLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetProvidersLinkPartsErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ListCollectionNames {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    NoSuchCollections {
        source: HashMap<ProviderKind, String>,
        where_was: WhereWas,
    },
    GetDocuments {
        source: HashMap<ProviderKind, Box<MongoGetDocumentsAsStringVectorErrorEnum>>,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_get_providers_link_parts(
) -> Result<HashMap<ProviderKind, Vec<String>>, MongoGetProvidersLinkPartsError> {
    match ClientOptions::parse(CONFIG.get_mongo_url()).await {
        Err(e) => Err(MongoGetProvidersLinkPartsError {
            source: Box::new(MongoGetProvidersLinkPartsErrorEnum::ClientOptionsParse {
                source: e,
                where_was: WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: tufa_common::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
                },
            }),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoGetProvidersLinkPartsError {
                source: Box::new(MongoGetProvidersLinkPartsErrorEnum::ClientWithOptions {
                    source: e,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                        git_info: tufa_common::common::where_was::GitInfoForWhereWas {
                            commit_id: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                            ),
                            repo_link: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                            ),
                            author: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.author,
                            ),
                            author_email: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO
                                    .author_email,
                            ),
                            commit_unix_time: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO
                                    .commit_unix_time,
                            ),
                            timezone: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                            ),
                            message: String::from(
                                crate::global_variables::compile_time::git_info::GIT_INFO.message,
                            ),
                        },
                    },
                }),
            }),
            Ok(client) => {
                let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
                match db.list_collection_names(None).await {
                    Err(e) => Err(MongoGetProvidersLinkPartsError {
                        source: Box::new(
                            MongoGetProvidersLinkPartsErrorEnum::ListCollectionNames {
                                source: e,
                                where_was:                 WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: tufa_common::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
                },
                            },
                        ),
                    }),
                    Ok(vec_collection_names) => {
                        let no_collection_error_hashmap = ProviderKind::get_enabled_providers_vec()
                            .into_iter()
                            .filter_map(|pk| {
                                let collection_name = pk.get_mongo_log_collection_name();
                                if !vec_collection_names.contains(&collection_name) {
                                    return Some((pk, collection_name));
                                }
                                None
                            })
                            .collect::<HashMap<ProviderKind, String>>();
                        if !no_collection_error_hashmap.is_empty() {
                            return Err(MongoGetProvidersLinkPartsError {
                                source: Box::new(
                                    MongoGetProvidersLinkPartsErrorEnum::NoSuchCollections {
                                        source: no_collection_error_hashmap,
                                        where_was:                 WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: tufa_common::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
                },
                                    },
                                ),
                            });
                        }
                        let result_get_documents_hashmap =
                                join_all(ProviderKind::get_enabled_providers_vec().iter().map(|pk| async {
                                    (
                                        *pk,
                                        mongo_get_documents_as_string_vector(
                                            db.collection::<Document>(&pk.get_mongo_log_collection_name()),
                                            &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                                            ProviderKind::get_mongo_provider_link_parts_aggregation(pk),
                                        )
                                        .await,
                                    )
                                }
                            ))
                            .await;
                        let mut success_hashmap: HashMap<ProviderKind, Vec<String>> =
                            HashMap::new();
                        let mut error_hashmap: HashMap<
                            ProviderKind,
                            Box<MongoGetDocumentsAsStringVectorErrorEnum>,
                        > = HashMap::new();
                        for (pk, result) in result_get_documents_hashmap.into_iter() {
                            match result {
                                Err(e) => {
                                    error_hashmap.insert(pk, e);
                                }
                                Ok(vec) => {
                                    success_hashmap.insert(pk, vec);
                                }
                            }
                        }
                        if !error_hashmap.is_empty() {
                            return Err(MongoGetProvidersLinkPartsError {
                                source: Box::new(
                                    MongoGetProvidersLinkPartsErrorEnum::GetDocuments {
                                        source: error_hashmap,
                                        where_was:                 WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: tufa_common::common::where_was::GitInfoForWhereWas {
                        commit_id: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.commit_id,
                        ),
                        repo_link: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.repo_link,
                        ),
                        author: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author,
                        ),
                        author_email: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.author_email,
                        ),
                        commit_unix_time: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO
                                .commit_unix_time,
                        ),
                        timezone: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                        ),
                        message: String::from(
                            crate::global_variables::compile_time::git_info::GIT_INFO.message,
                        ),
                    },
                },
                                    },
                                ),
                            });
                        }
                        Ok(success_hashmap)
                    }
                }
            }
        },
    }
}
