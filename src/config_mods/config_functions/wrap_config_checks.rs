extern crate toml;

use crate::traits::wrap_config_checks_trait::WrapConfigChecks;

use crate::config_mods::config_struct::ConfigStruct;

#[derive(Debug)]
pub struct WrapConfigChecksError {
    pub source: Box<WrapConfigChecksErrorEnum>,
}

#[derive(Debug)]
pub enum WrapConfigChecksErrorEnum {
    GithubName {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    GithubToken {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    RedditUserAgent {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    RedditClientId {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    RedditClientSecret {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    RedditUsername {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    RedditPassword {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoLogin {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoPassword {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoIp {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    MongoPort {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    LogFileExtension {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    PathToProviderLinkPartsFolder {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    ProvidersDbCollectionDocumentFieldName {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    WarningLogsDirectoryName {
        source: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    LinksLimitProviderse {
        source: i64,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

impl WrapConfigChecks for ConfigStruct {
    fn wrap_config_checks(self) -> Result<Self, WrapConfigChecksError> {
        if !self.github_name.is_empty() {
                return Err(WrapConfigChecksError {
                    source: Box::new(WrapConfigChecksErrorEnum::GithubName {
                        source: self.github_name,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    })
                });
        }
        if !self.github_token.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::GithubToken {
                    source: self.github_token,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .reddit_user_agent
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditUserAgent {
                    source: self.reddit_user_agent,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .reddit_client_id
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditClientId {
                    source: self.reddit_client_id,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .reddit_client_secret
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditClientSecret {
                    source: self.reddit_client_secret,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .reddit_username
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditUsername {
                    source: self.reddit_username,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .reddit_password
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::RedditPassword {
                    source: self.reddit_password,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .mongo_login
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoLogin {
                    source: self.mongo_login,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .mongo_password
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoPassword {
                    source: self.mongo_password,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .mongo_ip
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoIp {
                    source: self.mongo_ip,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if !self
            .mongo_port
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::MongoPort {
                    source: self.mongo_port,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if self.log_file_extension.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::LogFileExtension {
                    source: self.log_file_extension,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if self
            .path_to_provider_link_parts_folder
            .is_empty()
        {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::PathToProviderLinkPartsFolder {
                    source: self.path_to_provider_link_parts_folder,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if self.warning_logs_directory_name.is_empty() {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::WarningLogsDirectoryName {
                    source: self.warning_logs_directory_name,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        if self.links_limit_providers > 0 {
            return Err(WrapConfigChecksError {
                source: Box::new(WrapConfigChecksErrorEnum::LinksLimitProviderse {
                    source: self.links_limit_providers,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            });
        }
        // if !ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&self) {
        //     return Err(WrapConfigChecksError {
        //         source: Box::new(WrapConfigChecksErrorEnum::GithubName {
        //             source: self.github_name,
        //             file: file!(),
        //             line: line!(),
        //             column: column!(),
        //         })
        //     });
        // }
        Ok(self)
    }
}
