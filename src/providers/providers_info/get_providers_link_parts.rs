use crate::mongo_integration::mongo_get_providers_link_parts::mongo_get_providers_link_parts;
use crate::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsWrapperError;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::server::resource::Resource;
// use crate::postgres_integration::postgres_get_providers_link_parts::postgres_get_providers_link_parts;
// use crate::postgres_integration::postgres_get_providers_link_parts::PostgresGetProviderLinksError;

#[derive(Debug)]
pub enum GetProvidersLinkPartsErrorEnum {
    Local {
        source: GetLocalProvidersLinkPartsWrapperError,
        where_was: WhereWas,
    },
    Mongodb {
        source: MongoGetProvidersLinkPartsError,
        where_was: WhereWas,
    },
    PostgreSql {
        // source: PostgresGetProviderLinksError,
        // where_was: WhereWas,
    },
}

pub async fn get_providers_link_parts(
    resource: &Resource,
) -> Result<HashMap<ProviderKind, Vec<String>>, Box<GetProvidersLinkPartsErrorEnum>> {
    match resource {
        Resource::Local => match get_local_providers_link_parts(false).await {
            Err(error_hashmap) => Err(Box::new(GetProvidersLinkPartsErrorEnum::Local {
                source: *error_hashmap,
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
            })),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        Resource::Mongodb => match mongo_get_providers_link_parts().await {
            Err(e) => Err(Box::new(GetProvidersLinkPartsErrorEnum::Mongodb {
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
            })),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        // Resource::PostgreSql => match postgres_get_providers_link_parts().await {
        //     Err(e) => Err(GetProvidersLinkPartsError {
        // source: Box::new(GetProvidersLinkPartsError::PostgreSql(e))
        // }),
        //     Ok(success_hashmap) => Ok(success_hashmap),
        // },
        Resource::PostgreSql => todo!(),
    }
}
