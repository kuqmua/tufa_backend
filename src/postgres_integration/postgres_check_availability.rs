use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing_for_original_error_struct_without_source_enum::InitErrorWithTracingForOriginalErrorStructWithoutSourceEnum;
use impl_get_source_for_original_error_struct::ImplGetSourceForOriginalErrorStruct;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::time::Duration;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceForOriginalErrorStruct,
    // ImplGetWhereWasForErrorStruct,
    InitError,
    InitErrorWithTracingForOriginalErrorStructWithoutSourceEnum,
)]
pub struct PostgresCheckAvailabilityError {
    source: Error,
    where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for PostgresCheckAvailabilityError
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

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_check_availability(
    postgres_url: &str,
    should_trace: bool,
) -> Result<(), Box<PostgresCheckAvailabilityError>> {
    if let Err(e) = PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout))
        .connect(postgres_url)
        .await
    {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(PostgresCheckAvailabilityError::with_tracing(
                    e, where_was,
                )));
            }
            false => {
                return Err(Box::new(PostgresCheckAvailabilityError::new(e, where_was)));
            }
        }
    }
    Ok(())
}
