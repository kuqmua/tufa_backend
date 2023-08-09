use sqlx::Row;
use tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect;

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
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => &tufa_common::server::postgres::constants::DEFAULT_POSTGRES_SELECT_LIMIT,
    };
    //
    let where_name = "WHERE";
    let mut where_handle = std::string::String::from("");
    if let Some(id) = query_parameters.id.clone() {
        match where_handle.is_empty() {
            true => {
                where_handle.push_str(&format!("{where_name} id = {id}"));
            }
            false => {
                where_handle.push_str(&format!(" AND id = {id}"));
            }
        }
    }
    if let Some(name) = query_parameters.name.clone() {
        match where_handle.is_empty() {
            true => {
                where_handle.push_str(&format!("{where_name} name = \'{name}\'"));
            }
            false => {
                where_handle.push_str(&format!(" AND name = \'{name}\'"));
            }
        }
    }
    if let Some(color) = query_parameters.color.clone() {
        match where_handle.is_empty() {
            true => {
                where_handle.push_str(&format!("{where_name} color = \'{color}\'"));
            }
            false => {
                where_handle.push_str(&format!(" AND color = \'{color}\'"));
            }
        }
    }
    println!("{where_handle}");
    // let select_string_parameters = match &query_parameters.select {
    //     Some(get_select) => match get_select {
    //         GetSelect::Id => std::string::String::from("$1"),
    //         GetSelect::Name => std::string::String::from("$1"),
    //         GetSelect::Color => std::string::String::from("$1"),
    //         GetSelect::IdName => std::string::String::from("$1, $2"),
    //         GetSelect::IdColor => std::string::String::from("$1, $2"),
    //         GetSelect::NameColor => std::string::String::from("$1, $2"),
    //         GetSelect::IdNameColor => std::string::String::from("$1, $2, $3"),
    //     },
    //     None => std::string::String::from("$1, $2, $3"),
    // };
    let select = GetSelect::from(query_parameters.select.clone());
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

    // // let f = f.bind(limit);
    // //
    //
    // async fn notify(pool: &PgPool, s: &str) -> Result<(), sqlx::Error> {
    //     sqlx::query(
    //         r#"
    // SELECT pg_notify(chan, payload)
    // FROM (VALUES ('chan0', $1)) v(chan, payload)
    // "#,
    //     )
    //     .bind(s)
    //     .execute(pool)
    //     .await?;

    //     Ok(())
    // }
    //
    // sqlx::query_as!(
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
    //     // tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions,
    //     "SELECT id FROM cats LIMIT ?",
    //     *limit as i64
    // )
    let query_string = format!("SELECT {select} FROM cats {where_handle} LIMIT $1"); //{limit} // WHERE name = $2   LIMIT $1{select}{where_handle}{limit}
    println!("{query_string}");
    // let query_result =
    // match sqlx::query_as::<
    //     sqlx::Postgres,
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
    // >(&query_string)
    // // .bind("id")
    // // .bind("test_name")
    // // .bind(limit)
    // //   sqlx::query(&query_string)
    // .fetch_all(&*app_info_state.get_postgres_pool())
    // .await
    // {
    //     Ok(value) => {
    //         //     Ok(
    //         //     value,

    //         //     // value.into_iter()
    //         //           // .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //         //           // .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()
    //         // )
    //         // value.iter().for_each(|f| {
    //         //     let option_id: Option<i64> = f.get("id");
    //         //     println!("option_id\n{option_id:#?}");
    //         //     let option_name: Option<std::string::String> = f.get("name");
    //         //     println!("option_name\n{option_name:#?}");
    //         //     let option_color: Result<std::string::String, _> = f.get("color");
    //         // });
    //         // todo!("1")
    //     }
    //     Err(e) => {
    //         // Err(e)
    //         todo!("2")
    //     }
    // };
    // println!("{query_result:#?}");
    println!("{select}");
    println!("{select:#?}");
    let query_result = match select {
        GetSelect::Id => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
            >(&query_string);
            // if let Some(name) = query_parameters.name {
            //     query = query.bind(name)
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         println!("i\n{i}");
            //         query = query.bind(i.to_string());
            //     }
            // }
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
        GetSelect::Name => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
            >(&query_string);
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         query = query.bind(i.to_string());
            //     }
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
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
        GetSelect::Color => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatColor,
            >(&query_string);
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         query = query.bind(i.to_string());
            //     }
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
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
        GetSelect::IdName => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdName,
            >(&query_string);
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         query = query.bind(i.to_string());
            //     }
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
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
        GetSelect::IdColor => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdColor,
            >(&query_string);
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         query = query.bind(i.to_string());
            //     }
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
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
        GetSelect::NameColor => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatNameColor,
            >(&query_string);
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         query = query.bind(i.to_string());
            //     }
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
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
        GetSelect::IdNameColor => {
            let mut query = sqlx::query_as::<
                sqlx::Postgres,
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdNameColor,
            >(&query_string);
            // if let Some(get_select) = query_parameters.select {
            //     for i in get_select.into_get_select_field_vec() {
            //         query = query.bind(i.to_string());
            //     }
            // }
            if let Some(limit) = query_parameters.limit {
                query = query.bind(limit)
            }
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
    // // //
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

    // let query_result = match (
    //     &query_parameters.name,
    //     &query_parameters.color,
    //     &query_parameters.select.unwrap(),
    // ) {
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
    //             "SELECT id FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     }
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,

    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     }
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatColor,
    //             "SELECT color FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdName,
    //             "SELECT id, name FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdColor,
    //             "SELECT id, color FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatNameColor,
    //             "SELECT name, color FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdNameColor,
    //             "SELECT id, name, color FROM cats LIMIT $1",
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         None,
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE color = $1 LIMIT $2",
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         None,
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 LIMIT $2",
    //             name,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    //     (
    //         Some(name),
    //         Some(color),
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
    //     ) => match sqlx::query_as!(
    //         tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
    //             "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
    //             name,
    //             color,
    //             *limit as i64
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     {
    //         Ok(value) => Ok(value.into_iter()
    //             .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
    //             .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
    //         Err(e) => Err(e)
    //     },
    // };

    match query_result {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::Desirable(
            value.into_iter().map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element)).collect()
            // value
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
    // todo!("3")
}
