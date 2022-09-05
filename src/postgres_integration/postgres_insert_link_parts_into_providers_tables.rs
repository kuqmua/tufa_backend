use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;

#[derive(Debug, ImplGetWhereWasForErrorStruct)]
pub struct PostgresInsertLinkPartsIntoProvidersTablesError {
    source: HashMap<ProviderKind, sqlx::Error>,
    where_was: WhereWas,
}

impl PostgresInsertLinkPartsIntoProvidersTablesError {
    pub fn with_tracing(source: HashMap<ProviderKind, sqlx::Error>, where_was: WhereWas) -> Self {
        let mut formatted = source
            .iter()
            .map(|(pk, error)| format!("{} {},", pk, error))
            .fold(String::from(""), |mut acc, elem| {
                acc.push_str(&elem);
                acc
            });
        if !formatted.is_empty() {
            formatted.pop();
        }
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(error = formatted, source_place = where_was.source_place(),);
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = formatted,
                    github_source_place = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = formatted);
            }
        }
        Self { source, where_was }
    }
}
//todo implement better type support for derive(InitError)
impl PostgresInsertLinkPartsIntoProvidersTablesError {
    pub fn new(source: HashMap<ProviderKind, sqlx::Error>, where_was: WhereWas) -> Self {
        Self { source, where_was }
    }
}

impl crate::traits::get_source::GetSource for PostgresInsertLinkPartsIntoProvidersTablesError {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => {
                let mut formatted = self
                    .source
                    .iter()
                    .map(|(pk, error)| format!("{} {},", pk, error))
                    .collect::<Vec<String>>()
                    .iter()
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(elem);
                        acc
                    });
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_insert_link_parts_into_providers_tables(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
    should_trace: bool,
) -> Result<(), Box<PostgresInsertLinkPartsIntoProvidersTablesError>> {
    let insertion_error_hashmap = join_all(providers_json_local_data_hashmap.iter().map(
        |(pk, string_vec)| async {
            let mut values_string = String::from("");
            for link_part in string_vec.clone() {
                values_string.push_str(&format!("('{link_part}'),"));
            }
            if !values_string.is_empty() {
                values_string.pop();
            }
            let query_string = format!(
                "INSERT INTO {} (link_part) VALUES {values_string};",
                pk.get_postgres_table_name()
            );
            (*pk, sqlx::query(&query_string).execute(pool).await)
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((pk, e));
        }
        None
    })
    .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !insertion_error_hashmap.is_empty() {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(
                    PostgresInsertLinkPartsIntoProvidersTablesError::with_tracing(
                        insertion_error_hashmap,
                        where_was,
                    ),
                ));
            }
            false => {
                return Err(Box::new(
                    PostgresInsertLinkPartsIntoProvidersTablesError::new(
                        insertion_error_hashmap,
                        where_was,
                    ),
                ));
            }
        }
    }
    Ok(())
}
