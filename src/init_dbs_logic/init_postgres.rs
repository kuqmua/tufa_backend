use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::models::insertable::insertable_arxiv_link_part::InsertableArxivLinkPart;
use crate::postgres_integration::models::insertable::insertable_biorxiv_link_part::InsertableBiorxivLinkPart;
use crate::postgres_integration::models::insertable::insertable_github_link_part::InsertableGithubLinkPart;
use crate::postgres_integration::models::insertable::insertable_habr_link_part::InsertableHabrLinkPart;
use crate::postgres_integration::models::insertable::insertable_medrxiv_link_part::InsertableMedrxivLinkPart;
use crate::postgres_integration::models::insertable::insertable_reddit_link_part::InsertableRedditLinkPart;
use crate::postgres_integration::models::insertable::insertable_twitter_link_part::InsertableTwitterLinkPart;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
// use crate::postgres_integration::schemas::providers_link_parts_schema::providers_link_parts::dsl::providers_link_parts;
use crate::providers::provider_kind_impl::provider_kind_trait::ProviderTables;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::postgres_integration::schemas::arxiv_link_parts_schema::arxiv_link_parts::dsl::arxiv_link_parts;
use crate::postgres_integration::schemas::biorxiv_link_parts_schema::biorxiv_link_parts::dsl::biorxiv_link_parts;
use crate::postgres_integration::schemas::github_link_parts_schema::github_link_parts::dsl::github_link_parts;
use crate::postgres_integration::schemas::habr_link_parts_schema::habr_link_parts::dsl::habr_link_parts;
use crate::postgres_integration::schemas::medrxiv_link_parts_schema::medrxiv_link_parts::dsl::medrxiv_link_parts;
use crate::postgres_integration::schemas::reddit_link_parts_schema::reddit_link_parts::dsl::reddit_link_parts;
use crate::postgres_integration::schemas::twitter_link_parts_schema::twitter_link_parts::dsl::twitter_link_parts;

#[derive(Debug)]
pub enum PostgresInitError {
    LoadingProvidersLinkParts(diesel::result::Error),
    ProvidersLinkPartsIsNotEmpty(i64),
    InsertPosts(diesel::result::Error),
    EstablishConnection(ConnectionError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitError> {
    match PgConnection::establish(&postgres_get_db_url()) {
        Err(e) => Err(PostgresInitError::EstablishConnection(e)),
        Ok(pg_connection) => {
            // Todo: all tables initialization as one transaction?
            for (pk, data_vec) in providers_json_local_data_hashmap {
                match pk {
                    ProviderKind::Arxiv => {
                        match pk.get_link_parts_count(&pg_connection) {
                            Err(e) => (),
                            // Err(PostgresInitError::LoadingProvidersLinkParts(e))
                            Ok(vec) => {
                                if vec.len() != 1 {
                                    panic!("find out why table.count().load vec.len() is not 1");
                                }
                                match vec.get(0) {
                                    Some(size) => {
                                        if *size > 0 {
                                            return Err(
                                                PostgresInitError::ProvidersLinkPartsIsNotEmpty(
                                                    *size,
                                                ),
                                            );
                                        }
                                    }
                                    _ => panic!("no first element"),
                                }
                                let posts_vec = data_vec.into_iter().map(|data|{
                                    InsertableArxivLinkPart {
                                        link_part: data,
                                    }
                                }).collect::<Vec<InsertableArxivLinkPart>>();
                                if let Err(e) = diesel::insert_into(arxiv_link_parts)
                                    .values(&posts_vec)
                                    .get_result::<QueryableLinkPart>(&pg_connection)
                                {
                                    // return Err(PostgresInitError::InsertPosts(e));
                                }
                            }
                        }
                    }
                    ProviderKind::Biorxiv => todo!(),
                    ProviderKind::Github => todo!(),
                    ProviderKind::Habr => todo!(),
                    ProviderKind::Medrxiv => todo!(),
                    ProviderKind::Reddit => todo!(),
                    ProviderKind::Twitter => todo!(),
                }
            }
            Ok(())
        }
    }
}   

impl ProviderKind {
    pub fn get_link_parts_count(&self, pg_connection: &PgConnection) -> Result<Vec<i64>, diesel::result::Error> {
        match self {
            ProviderKind::Arxiv => arxiv_link_parts.count().load(pg_connection),
            ProviderKind::Biorxiv => biorxiv_link_parts.count().load(pg_connection),
            ProviderKind::Github => github_link_parts.count().load(pg_connection),
            ProviderKind::Habr => habr_link_parts.count().load(pg_connection),
            ProviderKind::Medrxiv => medrxiv_link_parts.count().load(pg_connection),
            ProviderKind::Reddit => reddit_link_parts.count().load(pg_connection),
            ProviderKind::Twitter => twitter_link_parts.count().load(pg_connection),
        }
    }
    // pub fn insert_provider_links_into_postgres(&self, pg_connection: &PgConnection) -> Result<(), diesel::result::Error> {
    //     match self {
    //         ProviderKind::Arxiv => diesel::insert_into(arxiv_link_parts)
    //         .values(&posts_vec)
    //         .get_result::<QueryableLinkPart>(&pg_connection),
    //         ProviderKind::Biorxiv => biorxiv_link_parts.count().load(pg_connection),
    //         ProviderKind::Github => github_link_parts.count().load(pg_connection),
    //         ProviderKind::Habr => habr_link_parts.count().load(pg_connection),
    //         ProviderKind::Medrxiv => medrxiv_link_parts.count().load(pg_connection),
    //         ProviderKind::Reddit => reddit_link_parts.count().load(pg_connection),
    //         ProviderKind::Twitter => twitter_link_parts.count().load(pg_connection),
    //     }
    // }
}