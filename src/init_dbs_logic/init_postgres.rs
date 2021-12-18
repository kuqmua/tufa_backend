use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::models::insertable::insertable_link_part::InsertableLinkPart;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts::dsl::*;

#[derive(Debug)]
pub enum PostgresInitDbError {
    LoadingProvidersLinkParts(diesel::result::Error),
    ProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    InsertPosts(diesel::result::Error),
    EstablishConnection(ConnectionError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitDbError> {
    match PgConnection::establish(&postgres_get_db_url()) {
        Err(e) => Err(PostgresInitDbError::EstablishConnection(e)),
        Ok(pg_connection) => {
            let result = providers_link_parts
                // .filter()
                // .limit(5)
                .load::<QueryableLinkPart>(&pg_connection);
            match result {
                Err(e) => Err(PostgresInitDbError::LoadingProvidersLinkParts(e)),
                Ok(vec) => {
                    if !vec.is_empty() {
                        return Err(PostgresInitDbError::ProvidersLinkPartsIsNotEmpty(vec));
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
                    let insertion_result =
                        InsertableLinkPart::insert_vec_into_postgres(&pg_connection, posts_vec);
                    if let Err(e) = insertion_result {
                        return Err(PostgresInitDbError::InsertPosts(e));
                    }
                    Ok(())
                }
            }
        }
    }
}
