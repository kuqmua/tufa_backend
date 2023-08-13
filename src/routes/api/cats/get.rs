pub(crate) async fn get(
    query_parameters_extraction_result: Result<
        axum::extract::Query<
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetQueryParameters,
        >,
        axum::extract::rejection::QueryRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let query_parameters = match tufa_common::server::routes::helpers::query_extractor_error::QueryValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetQueryParameters,
        tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants
    >::try_extract_value(
        query_parameters_extraction_result,
        &app_info_state
    ) {
        Ok(query_parameters) => query_parameters,
        Err(err) => {
            return err;
        },
    };
    println!(
        "get query_parameters limit {:?}, id {:?} name {:?} color {:?} select {:?}",
        query_parameters.limit,
        query_parameters.id,
        query_parameters.name,
        query_parameters.color,
        query_parameters.select
    );
    let mut additional_parameters = std::string::String::from("");
    let mut increment: u64 = 0;
    if let Some(value) = &query_parameters.id {
        let prefix = match additional_parameters.is_empty() {
            true => format!("{}", tufa_common::server::postgres::constants::WHERE_NAME),
            false => format!(" {}", tufa_common::server::postgres::constants::AND_NAME),
        };
        additional_parameters.push_str(&format!(
            "{prefix} id = {}({}[{}])", 
            tufa_common::server::postgres::constants::ANY_NAME,
            tufa_common::server::postgres::constants::ARRAY_NAME,
            tufa_common::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
    }
    if let Some(value) = &query_parameters.name {
        let prefix = match additional_parameters.is_empty() {
            true => format!("{}", tufa_common::server::postgres::constants::WHERE_NAME),
            false => format!(" {}", tufa_common::server::postgres::constants::AND_NAME),
        };
        additional_parameters.push_str(&format!(
            "{prefix} name = {}({}[{}])",
            tufa_common::server::postgres::constants::ANY_NAME,
            tufa_common::server::postgres::constants::ARRAY_NAME,
            tufa_common::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
    }
    if let Some(value) = &query_parameters.color {
        let prefix = match additional_parameters.is_empty() {
            true => format!("{}", tufa_common::server::postgres::constants::WHERE_NAME),
            false => format!(" {}", tufa_common::server::postgres::constants::AND_NAME),
        };
        additional_parameters.push_str(&format!(
            "{prefix} color = {}({}[{}])",
            tufa_common::server::postgres::constants::ANY_NAME,
            tufa_common::server::postgres::constants::ARRAY_NAME,
            tufa_common::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
    }
    {
        increment += 1;
        let limit_prefix = match additional_parameters.is_empty() {
            true => format!("{}", tufa_common::server::postgres::constants::LIMIT_NAME),
            false => format!(" {}", tufa_common::server::postgres::constants::LIMIT_NAME),
        };
        additional_parameters.push_str(&format!("{limit_prefix} ${increment}"));
    }
    let select = tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::from(
        query_parameters.select.clone(),
    );
    let query_string = format!(
        "{} {select} {} {} {additional_parameters}",
        tufa_common::server::postgres::constants::SELECT_NAME,
        tufa_common::server::postgres::constants::FROM_NAME,
        tufa_common::repositories_types::tufa_server::routes::api::cats::CATS
    );
    println!("{query_string}");
    select
        .execute_query(
            query_string,
            &*app_info_state.get_postgres_pool(),
            query_parameters,
            &app_info_state,
        )
        .await
}
