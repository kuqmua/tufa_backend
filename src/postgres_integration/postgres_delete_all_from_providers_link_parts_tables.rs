use crate::lazy_static::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;

#[derive(Debug, InitError)] //, ImplGetWhereWasForErrorStruct
pub struct PostgresDeleteAllFromProvidersTablesError {
    pub source: HashMap<ProviderKind, sqlx::Error>,
    pub where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for PostgresDeleteAllFromProvidersTablesError
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}

impl tufa_common::traits::with_tracing::WithTracing<HashMap<ProviderKind, sqlx::Error>>
    for PostgresDeleteAllFromProvidersTablesError
{
    fn with_tracing(source: HashMap<ProviderKind, sqlx::Error>, where_was: WhereWas) -> Self {
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
        match crate::lazy_static::config::CONFIG.source_place_type {
            tufa_common::config::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = formatted,
                    source_place = where_was.file_line_column(),
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = formatted,
                    github_source_place = where_was
                        .github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = formatted);
            }
        }
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_source::GetSource for PostgresDeleteAllFromProvidersTablesError {
    fn get_source(&self) -> String {
        match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
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
pub async fn postgres_delete_all_from_providers_link_parts_tables(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
    should_trace: bool,
) -> Result<(), Box<PostgresDeleteAllFromProvidersTablesError>> {
    let delete_from_tables_error_hashmap =
        join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
            let query_string = format!("DELETE FROM {} ;", pk.get_postgres_table_name());
            (*pk, sqlx::query(&query_string).execute(pool).await)
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| {
            if let Err(e) = result {
                return Some((pk, e));
            }
            None
        })
        .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !delete_from_tables_error_hashmap.is_empty() {
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
                    PostgresDeleteAllFromProvidersTablesError::with_tracing(
                        delete_from_tables_error_hashmap,
                        where_was,
                    ),
                ));
            }
            false => {
                return Err(Box::new(PostgresDeleteAllFromProvidersTablesError::new(
                    delete_from_tables_error_hashmap,
                    where_was,
                )));
            }
        }
    }
    Ok(())
}
