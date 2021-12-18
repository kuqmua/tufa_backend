use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts::dsl::*;

use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;

#[derive(Debug)]
pub enum PostgresGetProviderLinksError {
    LoadingProvidersLinkParts(diesel::result::Error),
    EstablishConnection(ConnectionError),
}

#[deny(clippy::indexing_slicing)]
pub async fn postgres_get_providers_link_parts(
) -> Result<HashMap<ProviderKind, Vec<String>>, PostgresGetProviderLinksError> {
    match PgConnection::establish(&postgres_get_db_url()) {
        Err(e) => Err(PostgresGetProviderLinksError::EstablishConnection(e)),
        Ok(pg_connection) => {
            let result = providers_link_parts
                // .filter()//todo
                .limit(CONFIG.common_providers_links_limit)
                .load::<QueryableLinkPart>(&pg_connection);
            match result {
                Err(e) => Err(PostgresGetProviderLinksError::LoadingProvidersLinkParts(e)),
                Ok(vec) => {
                    let mut providers_vec_from_db = Vec::new();
                    //trashcode warning
                    for i in &vec {
                        if i.provider_kind == "arxiv"
                            || i.provider_kind == "biorxiv"
                            || i.provider_kind == "github"
                            || i.provider_kind == "habr"
                            || i.provider_kind == "medrxiv"
                            || i.provider_kind == "reddit"
                            || i.provider_kind == "twitter"
                        {
                            if i.provider_kind == "arxiv" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Arxiv)) {
                                    providers_vec_from_db.push(ProviderKind::Arxiv);
                                }
                            } else if i.provider_kind == "biorxiv" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Biorxiv)) {
                                    providers_vec_from_db.push(ProviderKind::Biorxiv);
                                }
                            } else if i.provider_kind == "github" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Github)) {
                                    providers_vec_from_db.push(ProviderKind::Github);
                                }
                            } else if i.provider_kind == "habr" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Habr)) {
                                    providers_vec_from_db.push(ProviderKind::Habr);
                                }
                            } else if i.provider_kind == "medrxiv" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Medrxiv)) {
                                    providers_vec_from_db.push(ProviderKind::Medrxiv);
                                }
                            } else if i.provider_kind == "reddit" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Reddit)) {
                                    providers_vec_from_db.push(ProviderKind::Reddit);
                                }
                            } else if i.provider_kind == "twitter" {
                                if !(providers_vec_from_db.contains(&ProviderKind::Twitter)) {
                                    providers_vec_from_db.push(ProviderKind::Twitter);
                                }
                            }
                        } else {
                            todo!()
                        }
                    }
                    let mut hashmap_to_return: HashMap<ProviderKind, Vec<String>> = HashMap::new(); //todo with capacity
                    for pk in providers_vec_from_db {
                        let mut vecc = Vec::new();
                        for i in &vec {
                            if i.provider_kind == format!("{}", pk) {
                                vecc.push(i.link_part.clone());
                            }
                        }
                        hashmap_to_return.insert(pk, vecc);
                    }
                    Ok(hashmap_to_return)
                }
            }
        }
    }
}
