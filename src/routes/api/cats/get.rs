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
    let where_name = "WHERE";
    let and_name = "AND";
    let limit_name = "LIMIT";
    let mut additional_parameters = std::string::String::from("");
    let mut increment: u64 = 0;
    if let Some(bigserial_ids) = &query_parameters.id {
        let increments = {
            let mut increments = std::string::String::from("");
            bigserial_ids.0.iter().for_each(|_| {
                increment += 1;
                increments.push_str(&format!("${increment}, "));
            });
            if let false = increments.is_empty() {
                increments.pop();
                increments.pop();
            }
            increments
        };
        let prefix = match additional_parameters.is_empty() {
            true => format!("{where_name}"),
            false => format!(" {and_name}"),
        };
        additional_parameters.push_str(&format!("{prefix} id = ANY(ARRAY[{increments}])"));
    }
    //
    if let Some(value) = &query_parameters.name {
        let increments = {
            let mut increments = std::string::String::from("");
            value.0.iter().for_each(|_| {
                increment += 1;
                increments.push_str(&format!("${increment}, "));
            });
            if let false = increments.is_empty() {
                increments.pop();
                increments.pop();
            }
            increments
        };
        let prefix = match additional_parameters.is_empty() {
            true => format!("{where_name}"),
            false => format!(" {and_name}"),
        };
        additional_parameters.push_str(&format!("{prefix} name = ANY(ARRAY[{increments}])"));
    }
    if let Some(value) = &query_parameters.color {
        let increments = {
            let mut increments = std::string::String::from("");
            value.0.iter().for_each(|_| {
                increment += 1;
                increments.push_str(&format!("${increment}, "));
            });
            if let false = increments.is_empty() {
                increments.pop();
                increments.pop();
            }
            increments
        };
        let prefix = match additional_parameters.is_empty() {
            true => format!("{where_name}"),
            false => format!(" {and_name}"),
        };
        additional_parameters.push_str(&format!("{prefix} color = ANY(ARRAY[{increments}])"));
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
    let query_string = format!("SELECT {select} FROM cats {additional_parameters}");
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

// #[derive(serde_derive::Deserialize, sqlx::FromRow)]
// struct IdWrapper { id: i64 }
// //1 works
// let id_handle = "id";
// sqlx::query_as::<sqlx::Postgres, IdWrapper>(&format!("SELECT {id_handle} FROM example")).fetch_all(postgres_pool).await
// //2 works
// sqlx::query_as!(IdWrapper, "SELECT id FROM example").fetch_all(postgres_pool).await
// //3 runtime error: column not found: id
// let mut query = sqlx::query_as::<sqlx::Postgres, IdWrapper>(&format!("SELECT $1 FROM example"));
// query = query.bind("id");
// query.fetch_all(postgres_pool).await
// //4 compile time error: column name "?column?" is invalid: "" is not a valid Rust identifier
// let id_handle = "id";
// sqlx::query_as!(IdWrapper, "SELECT $1 FROM example", id_handle).fetch_all(postgres_pool).await
