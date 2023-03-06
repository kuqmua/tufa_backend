use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_methods::ProviderKindMethods;
use futures::future::join_all;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

pub async fn postgres_insert_link_parts_into_providers_tables<'a>(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginError<'a>>>{
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
            return Some((
                pk.to_string(), 
                tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnum::Postgres(
                    tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumError::Postgres { 
                        error: e, 
                        code_occurence: tufa_common::code_occurence!()
                    }
                )
            ));
        }
        None
    })
    .collect::<HashMap<
        String, tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnum
    >>();
    if !insertion_error_hashmap.is_empty() {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginError::Postgres { 
                inner_errors: insertion_error_hashmap, 
                code_occurence: tufa_common::code_occurence!() 
            }
        ));
    }
    Ok(())
}
