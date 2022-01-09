use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::models::insertable::insertable_link_part::InsertableLinkPart;
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
            for (pk, data_vec) in providers_json_local_data_hashmap {
                match pk {
                    ProviderKind::Arxiv => {
                        let result: Result<Vec<i64>, diesel::result::Error> =
                            arxiv_link_parts.count().load(&pg_connection);
                        match result {
                            Err(e) =>
                            // Err(PostgresInitError::LoadingProvidersLinkParts(e))
                            {
                                ()
                            }
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
                                let mut posts_vec: Vec<InsertableLinkPart> =
                                    Vec::with_capacity(data_vec.len());
                                for data in data_vec {
                                    posts_vec.push(InsertableLinkPart {
                                        provider_kind: format!("{}", pk.clone()),
                                        link_part: data.clone(),
                                    });
                                }
                                if let Err(e) = diesel::insert_into(arxiv_link_parts)
                                    .values(&posts_vec)
                                    .get_result::<QueryableLinkPart>(&pg_connection)
                                {
                                    // return Err(PostgresInitError::InsertPosts(e));
                                }
                                // Ok(())
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
