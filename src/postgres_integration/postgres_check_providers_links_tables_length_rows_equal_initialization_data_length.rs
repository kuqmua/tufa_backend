use tufa_common::repositories_types::tufa_server::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed;
use tufa_common::repositories_types::tufa_server::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed;
use tufa_common::repositories_types::tufa_server::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed;
use tufa_common::repositories_types::tufa_server::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed;
use tufa_common::repositories_types::tufa_server::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed;

pub async fn postgres_check_providers_links_tables_length_rows_equal_initialization_data_length<'a>(
    providers_json_local_data_hashmap: &std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    db: &sqlx::Pool<sqlx::Postgres>,
    should_trace: bool,
) -> Result<
    (),
    Box<PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed<'a>>,
>{
    let count_provider_links_tables_tasks_vec =
        providers_json_local_data_hashmap
            .iter()
            .map(|(pk, string_vec)| async move {
                let query_string = format!(
                    "SELECT count(*) AS exact_count FROM {};",
                    {
                        use tufa_common::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                        pk.get_postgres_table_name()
                    }
                );
                (
                    pk,
                    string_vec,
                    sqlx::query_as(&query_string).fetch_one(db).await,
                )
            });
    let count_provider_links_tables_error_vec: Vec<(
        &tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind,
        &Vec<String>,
        Result<(i64,), sqlx::Error>,
    )> = futures::future::join_all(count_provider_links_tables_tasks_vec).await;
    let mut count_provider_links_tables_error_hashmap: std::collections::HashMap<std::string::String, PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed> =
        std::collections::HashMap::new();
    let mut provider_links_tables_rows_length_not_equal_error_hashmap: std::collections::HashMap<
        std::string::String,
        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed,
    > = std::collections::HashMap::new();
    for (pk, string_vec, result) in count_provider_links_tables_error_vec {
        match result {
            Err(e) => {
                count_provider_links_tables_error_hashmap.insert(
                    pk.to_string(), 
                 PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxUnnamed::Postgres(
                        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorSqlxNamed::Postgres {
                            error: e,
                            code_occurence: tufa_common::code_occurence!()
                        }
                    )
                );
            }
            Ok((count,)) => {
                if count != string_vec.len() as i64 {
                    provider_links_tables_rows_length_not_equal_error_hashmap.insert(
                        pk.to_string(),
                        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorUnnamed::Postgres(
                            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorSelectCountOriginErrorNamed::Postgres {
                                table_rows_length: count,
                                initialization_data_length: string_vec.len(),
                                code_occurence: tufa_common::code_occurence!()
                            }
                        )
                    );
                }
            }
        }
    }
    if !count_provider_links_tables_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed::SelectCountOrigin {
                inner_errors: count_provider_links_tables_error_hashmap,
                code_occurence: tufa_common::code_occurence!()
            }
        ));
    }
    if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperErrorNamed::ProviderLinksTablesRowsLengthNotEqualOrigin {
                inner_errors: provider_links_tables_rows_length_not_equal_error_hashmap,
                code_occurence: tufa_common::code_occurence!()
            }
        ));
    }
    Ok(())
}
