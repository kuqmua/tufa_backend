pub async fn postgres_insert_link_parts_into_providers_tables<'a>(
    providers_json_local_data_hashmap: &std::collections::HashMap<crate::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    pool: &sqlx::Pool<sqlx::Postgres>,
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed<'a>>>{
    let insertion_error_hashmap = futures::future::join_all(providers_json_local_data_hashmap.iter().map(
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
                {
                    use crate::traits::provider_kind_methods::ProviderKindMethods;
                    pk.get_postgres_table_name()
                }
            );
            (*pk, sqlx::query(&query_string).execute(pool).await)
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((
                pk.to_string(), 
                tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed::PostgresInsertLinkPartsIntoProvidersTablesOriginHandle(
                    tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginHandleErrorNamed::Postgres { 
                        error: e, 
                        code_occurence: tufa_common::code_occurence!()
                    }
                )
            ));
        }
        None
    })
    .collect::<std::collections::HashMap<
        String, tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed
    >>();
    if !insertion_error_hashmap.is_empty() {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed::Postgres { 
                inner_errors: insertion_error_hashmap, 
                code_occurence: tufa_common::code_occurence!() 
            }
        ));
    }
    Ok(())
}
