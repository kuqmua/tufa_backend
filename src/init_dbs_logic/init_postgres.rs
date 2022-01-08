use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::models::insertable::insertable_link_part::InsertableLinkPart;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schemas::providers_link_parts_schema::providers_link_parts::dsl::*;

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
            //todo: maybe its not i64?
            let result: Result<Vec<i64>, diesel::result::Error> =
                providers_link_parts.count().load(&pg_connection);
            match result {
                Err(e) => Err(PostgresInitError::LoadingProvidersLinkParts(e)),
                Ok(vec) => {
                    if vec.len() != 1 {
                        panic!("find out why providers_link_parts.count().load vec.len() is not 1");
                    }
                    if vec[0] > 0 {
                        return Err(PostgresInitError::ProvidersLinkPartsIsNotEmpty(vec[0]));
                    }
                    let mut posts_vec: Vec<InsertableLinkPart> =
                        Vec::with_capacity(providers_json_local_data_hashmap.len());
                    for (provider_kind_handle, data_vec) in providers_json_local_data_hashmap {
                        for data in data_vec {
                            posts_vec.push(InsertableLinkPart {
                                provider_kind: format!("{}", provider_kind_handle.clone()),
                                link_part: data.clone(),
                            });
                        }
                    }
                    if let Err(e) = diesel::insert_into(providers_link_parts)
                        .values(&posts_vec)
                        .get_result::<QueryableLinkPart>(&pg_connection)
                    {
                        return Err(PostgresInitError::InsertPosts(e));
                    }
                    Ok(())
                }
            }
        }
    }
}
