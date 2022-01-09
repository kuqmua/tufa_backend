use std::collections::hash_map::Keys;

use crate::postgres_integration::schemas::arxiv_link_parts_schema::arxiv_link_parts::dsl::arxiv_link_parts;
use crate::postgres_integration::schemas::biorxiv_link_parts_schema::biorxiv_link_parts::dsl::biorxiv_link_parts;
use crate::postgres_integration::schemas::github_link_parts_schema::github_link_parts::dsl::github_link_parts;
use crate::postgres_integration::schemas::habr_link_parts_schema::habr_link_parts::dsl::habr_link_parts;
use crate::postgres_integration::schemas::medrxiv_link_parts_schema::medrxiv_link_parts::dsl::medrxiv_link_parts;
use crate::postgres_integration::schemas::reddit_link_parts_schema::reddit_link_parts::dsl::reddit_link_parts;
use crate::postgres_integration::schemas::twitter_link_parts_schema::twitter_link_parts::dsl::twitter_link_parts;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::provider_kind_trait::ProviderTables;

fn get_initialization_postgres_tables_vec_from_keys(
    keys: Keys<ProviderKind, Vec<String>>,
) -> Vec<ProviderTables> {
    let mut postgres_tables_vec = Vec::new();
    for pk in keys {
        match pk {
            ProviderKind::Arxiv => {
                postgres_tables_vec.push(ProviderTables::Arxiv(arxiv_link_parts))
            }
            ProviderKind::Biorxiv => {
                postgres_tables_vec.push(ProviderTables::Biorxiv(biorxiv_link_parts))
            }
            ProviderKind::Github => {
                postgres_tables_vec.push(ProviderTables::Github(github_link_parts))
            }
            ProviderKind::Habr => postgres_tables_vec.push(ProviderTables::Habr(habr_link_parts)),
            ProviderKind::Medrxiv => {
                postgres_tables_vec.push(ProviderTables::Medrxiv(medrxiv_link_parts))
            }
            ProviderKind::Reddit => {
                postgres_tables_vec.push(ProviderTables::Reddit(reddit_link_parts))
            }
            ProviderKind::Twitter => {
                postgres_tables_vec.push(ProviderTables::Twitter(twitter_link_parts))
            }
        }
    }
    postgres_tables_vec
}
