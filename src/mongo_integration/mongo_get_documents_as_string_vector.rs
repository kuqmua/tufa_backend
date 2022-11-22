use futures::stream::TryStreamExt;
use itertools::Itertools;
use mongodb::bson::Document;
use mongodb::Collection;
use tufa_common::common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoGetDocumentsAsStringVectorErrorEnum {
    CollectionAggregate {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CursorTryNext {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    WrongBsonType {
        source: mongodb::bson::Bson,
        where_was: WhereWas,
    },
    NoKeyInDocument {
        source: String,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<Document>,
) -> Result<Vec<String>, Box<MongoGetDocumentsAsStringVectorErrorEnum>> {
    match collection.aggregate(option_aggregation, None).await {
        Err(e) => Err(Box::new(
            MongoGetDocumentsAsStringVectorErrorEnum::CollectionAggregate {
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
            },
        )),
        Ok(mut cursor) => {
            let mut vec_of_strings: Vec<String> = Vec::new();
            loop {
                match cursor.try_next().await {
                    Err(e) => {
                        return Err(Box::new(
                            MongoGetDocumentsAsStringVectorErrorEnum::CursorTryNext {
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
                        ));
                    }
                    Ok(option_document) => match option_document {
                        None => {
                            break;
                        }
                        Some(document) => {
                            match document.get(db_collection_document_field_name_handle) {
                                    None => return Err(Box::new(
                                        MongoGetDocumentsAsStringVectorErrorEnum::NoKeyInDocument {
                                            source: db_collection_document_field_name_handle
                                                .to_string(),
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
                                    )),
                                    Some(bson_handle) => match bson_handle {
                                        mongodb::bson::Bson::String(value) => {
                                            vec_of_strings.push(value.to_string());
                                        }
                                        other_bson_type => {
                                            return Err(Box::new(
                                            MongoGetDocumentsAsStringVectorErrorEnum::WrongBsonType {
                                                source: other_bson_type.clone(),
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
                                            },
                                        ));
                                        }
                                    },
                                }
                        }
                    },
                }
            }
            Ok(vec_of_strings.into_iter().unique().collect())
        }
    }
}
