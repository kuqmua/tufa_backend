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
    let limit = &query_parameters.limit;
    let where_name = "WHERE";
    let and_name = "AND";
    let mut where_handle_increment = std::string::String::from("");
    let mut increment: u64 = 0;
    if let Some(id) = &query_parameters.id {
        match where_handle_increment.is_empty() {
            true => {
                increment += 1;
                where_handle_increment.push_str(&format!("{where_name} id = ${increment}"));
            }
            false => {
                increment += 1;
                where_handle_increment.push_str(&format!(" {and_name} id = ${increment}"));
            }
        }
    }
    if let Some(name) = &query_parameters.name {
        match where_handle_increment.is_empty() {
            true => {
                increment += 1;
                where_handle_increment.push_str(&format!("{where_name} name = ${increment}"));
            }
            false => {
                increment += 1;
                where_handle_increment.push_str(&format!(" {and_name} name = ${increment}"));
            }
        }
    }
    if let Some(color) = &query_parameters.color {
        match where_handle_increment.is_empty() {
            true => {
                increment += 1;
                where_handle_increment.push_str(&format!("{where_name} color = ${increment}"));
            }
            false => {
                increment += 1;
                where_handle_increment.push_str(&format!(" {and_name} color = ${increment}"));
            }
        }
    }
    // println!("where_handle {where_handle}");
    println!("where_handle_increment {where_handle_increment}");
    let select = tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::from(
        query_parameters.select.clone(),
    );
    // // WHERE color = $1
    // // WHERE some_id = ANY(ARRAY[1, 2])
    // // WHERE name = $1 AND color = $2

    // // let v = vec![1, 2];
    // // let params = format!("?{}", ", ?".repeat(v.len() - 1));
    // // let query_str = format!("SELECT id FROM test_table WHERE id IN ( { } )", params);
    // let mut query = sqlx::query(&query_str);
    // for i in v {
    //     query = query.bind(i);
    // }
    // // let row = query.fetch_all(&pool).await?;
    // //
    // // let params = format!("?{}", ", ?".repeat(v.len() - 1));
    //{select_string_parameters}
    increment += 1;
    let query_string =
        format!("SELECT {select} FROM cats {where_handle_increment} LIMIT ${increment}"); //{limit} // WHERE name = $2   LIMIT $1{select}{where_handle}{limit}
    println!("{query_string}");
    println!("{select}");
    println!("{select:#?}");
    let query_result = match select {
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatColor,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdName,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdColor,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatNameColor,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdNameColor,
            >(&query_string);
            if let Some(id) = query_parameters.id.clone() {
                query = query.bind(id.inner().clone());
            }
            if let Some(name) = query_parameters.name.clone() {
                query = query.bind(name);
            }
            if let Some(color) = query_parameters.color.clone() {
                query = query.bind(color);
            }
            query = query.bind(limit);
            match query
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
    // // Create a dynamic query string with the right number of parameter
    // // placeholders injected
    // // let query = format!(
    // //     "SELECT * FROM projects WHERE uuid IN ({})",
    // //     (0..keys.len())
    // //         .map(|_| "?")
    // //         .collect::<Vec<&str>>()
    // //         .join(",")
    // // );
    // // // Dynamically bind each entry from
    // // let mut q = sqlx::query_as::<sqlx::Sqlite, Project>(&query_string);
    // // for x in (0..uuids.len()) {
    // //     q = q.bind(uuids[x]);
    // // }
    // // let records = q.fetch(&conn).await?;
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
