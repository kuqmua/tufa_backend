use hmac::digest::typenum::array;

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
    let select_name = "SELECT";
    let from_name = "FROM";
    let where_name = "WHERE";
    let and_name = "AND";
    let limit_name = "LIMIT";
    let any_name = "ANY";
    let array_name = "ARRAY";
    let mut additional_parameters = std::string::String::from("");
    let mut increment: u64 = 0;
    if let Some(value) = &query_parameters.id {
        let prefix = match additional_parameters.is_empty() {
            true => format!("{where_name}"),
            false => format!(" {and_name}"),
        };
        additional_parameters.push_str(&format!(
            "{prefix} id = {any_name}({array_name}[{}])", 
            tufa_common::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
    }
    if let Some(value) = &query_parameters.name {
        let prefix = match additional_parameters.is_empty() {
            true => format!("{where_name}"),
            false => format!(" {and_name}"),
        };
        additional_parameters.push_str(&format!(
            "{prefix} name = {any_name}({array_name}[{}])",
            tufa_common::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
    }
    if let Some(value) = &query_parameters.color {
        let prefix = match additional_parameters.is_empty() {
            true => format!("{where_name}"),
            false => format!(" {and_name}"),
        };
        additional_parameters.push_str(&format!(
            "{prefix} color = {any_name}({array_name}[{}])",
            tufa_common::server::postgres::generate_bind_increments::GenerateBindIncrements::generate_bind_increments(value, &mut increment)
        ));
    }
    {
        increment += 1;
        let limit_prefix = match additional_parameters.is_empty() {
            true => format!("{limit_name}"),
            false => format!(" {limit_name}"),
        };
        additional_parameters.push_str(&format!("{limit_prefix} ${increment}"));
    }
    let select = tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::from(
        query_parameters.select.clone(),
    );
    let query_string = format!(
        "{select_name} {select} {from_name} {} {additional_parameters}",
        tufa_common::repositories_types::tufa_server::routes::api::cats::CATS
    );
    println!("{query_string}");
    let query_result = match select {
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdName,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
             .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdColor,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdName,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdColor,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatNameColor,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor => {
            match tufa_common::server::routes::helpers::bind_sqlx_query::BindSqlxQuery::bind_sqlx_query(query_parameters, sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdNameColor,
            >(&query_string))
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
            {
                Ok(value) => Ok(value.into_iter()
                    .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                    .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
                Err(e) => Err(e),
            }
        }
    };
    match query_result {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::Desirable(
            value.into_iter().map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element)).collect()
        ),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGet::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error)
        }
    }
}
