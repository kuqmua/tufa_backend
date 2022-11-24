use crate::global_variables::runtime::config::CONFIG;
use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
use crate::mongo_integration::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorEnum;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::common::where_was::WhereWas;
use tufa_common::config_mods::traits::get_mongo_url_trait::GetMongoUrl;

#[derive(Debug)]
pub struct MongoGetProviderLinkPartsError {
    pub source: Box<MongoGetProviderLinkPartsErrorEnum>,
}

#[derive(Debug)]
pub enum MongoGetProviderLinkPartsErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    MongoGetDocumentsAsStringVector {
        source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
        where_was: WhereWas,
    },
}

impl ProviderKind {
    //rust does not support async traits yet (end of 2021). only with third party crate
    pub async fn mongo_get_provider_link_parts(
        pk: ProviderKind,
    ) -> Result<Vec<String>, MongoGetProviderLinkPartsError> {
        match ClientOptions::parse(CONFIG.get_mongo_url()).await {
            Err(e) => Err(MongoGetProviderLinkPartsError {
                source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientOptionsParse {
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
            Ok(client_options) => match Client::with_options(client_options) {
                Err(e) => Err(MongoGetProviderLinkPartsError {
                    source: Box::new(MongoGetProviderLinkPartsErrorEnum::ClientWithOptions {
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
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .commit_id,
                                ),
                                repo_link: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .repo_link,
                                ),
                                author: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .author,
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
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .timezone,
                                ),
                                message: String::from(
                                    crate::global_variables::compile_time::git_info::GIT_INFO
                                        .message,
                                ),
                            },
                        },
                    }),
                }),
                Ok(client) => {
                    match mongo_get_documents_as_string_vector(
                        client
                            .database(&CONFIG.mongo_providers_logs_db_name)
                            .collection::<Document>(&pk.get_mongo_log_collection_name()),
                        &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                        ProviderKind::get_mongo_provider_link_parts_aggregation(&pk),
                    )
                    .await
                    {
                        Err(e) => Err(MongoGetProviderLinkPartsError {
                            source: Box::new(
                                MongoGetProviderLinkPartsErrorEnum::MongoGetDocumentsAsStringVector {
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
                        crate::global_variables::compile_time::git_info::GIT_INFO.commit_unix_time,
                    ),
                    timezone: String::from(
                        crate::global_variables::compile_time::git_info::GIT_INFO.timezone,
                    ),
                    message: String::from(
                        crate::global_variables::compile_time::git_info::GIT_INFO.message,
                    ),
                },
            },
                        })}),
                        Ok(vec_of_strings) => Ok(vec_of_strings),
                    }
                }
            },
        }
    }
}
