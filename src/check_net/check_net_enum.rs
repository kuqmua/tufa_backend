use std::fmt;

use strum_macros::EnumIter;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::check_net::check_net_availability::check_net_availability;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;

use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorEnum;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;

#[derive(Debug, EnumIter)]
pub enum CheckNet {
    Net,
    Postgres,
    Mongo,
}

#[derive(Debug)]
pub enum CheckNetError {
    Net(CheckNetAvailabilityErrorEnum),
    Postgres(PostgresCheckAvailabilityError),
    Mongo(MongoCheckAvailabilityErrorEnum),
}

impl fmt::Display for CheckNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckNetError::Net(e) => match e {
                CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError { source, where_was } => {
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        write!(
                            f,
                            "{}\n{}\n{}",
                            where_was.source_place_with_readable_time(),
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_github_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else {
                        write!(f, "{}", source)
                    }
                }
                CheckNetAvailabilityErrorEnum::StatusCodeError { source, where_was } => {
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        write!(
                            f,
                            "{}\n{}\n{}",
                            where_was.source_place_with_readable_time(),
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_github_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else {
                        write!(f, "{}", source)
                    }
                }
            },
            CheckNetError::Postgres(e) => {
                if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                {
                    write!(
                        f,
                        "{}\n{}\n{}",
                        e.where_was.source_place_with_readable_time(),
                        e.where_was.github_source_place_with_readable_time(),
                        e.source
                    )
                } else if CONFIG.is_show_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        e.where_was.source_place_with_readable_time(),
                        e.source
                    )
                } else if CONFIG.is_show_github_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        e.where_was.github_source_place_with_readable_time(),
                        e.source
                    )
                } else {
                    write!(f, "{}", e.source)
                }
            }
            CheckNetError::Mongo(e) => match e {
                MongoCheckAvailabilityErrorEnum::ClientOptionsParse { source, where_was } => {
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        write!(
                            f,
                            "{}\n{}\n{}",
                            where_was.source_place_with_readable_time(),
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_github_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else {
                        write!(f, "{}", source)
                    }
                }
                MongoCheckAvailabilityErrorEnum::ClientWithOptions { source, where_was } => {
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        write!(
                            f,
                            "{}\n{}\n{}",
                            where_was.source_place_with_readable_time(),
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_github_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else {
                        write!(f, "{}", source)
                    }
                }
                MongoCheckAvailabilityErrorEnum::ListCollectionNames { source, where_was } => {
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        write!(
                            f,
                            "{}\n{}\n{}",
                            where_was.source_place_with_readable_time(),
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.source_place_with_readable_time(),
                            source
                        )
                    } else if CONFIG.is_show_github_source_place_enabled {
                        write!(
                            f,
                            "{}\n{}",
                            where_was.github_source_place_with_readable_time(),
                            source
                        )
                    } else {
                        write!(f, "{}", source)
                    }
                }
            },
        }
    }
}

impl CheckNet {
    pub fn get_url(&self) -> String {
        match self {
            CheckNet::Net => CONFIG.starting_check_link.clone(),
            CheckNet::Mongo => postgres_get_db_url(),
            CheckNet::Postgres => mongo_get_db_url(),
        }
    }
    pub async fn check(self) -> Result<(), CheckNetError> {
        match self {
            CheckNet::Net => {
                if let Err(e) = check_net_availability(&self.get_url()).await {
                    return Err(CheckNetError::Net(*e));
                }
            }
            CheckNet::Postgres => {
                if let Err(e) = postgres_check_availability(&self.get_url()).await {
                    return Err(CheckNetError::Postgres(*e));
                }
            }
            CheckNet::Mongo => {
                if let Err(e) = mongo_check_availability(&self.get_url()).await {
                    return Err(CheckNetError::Mongo(*e));
                }
            }
        }
        Ok(())
    }
}
