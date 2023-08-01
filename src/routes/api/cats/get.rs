pub(crate) async fn get(
    query_parameters_result: Result<
        axum::extract::Query<
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetQueryParameters,
        >,
        axum::extract::rejection::QueryRejection,
    >,
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let axum::extract::Query(query_parameters) = match query_parameters_result {
        Ok(query_parameters) => query_parameters,
        Err(err) => {
            let error = tufa_common::server::routes::helpers::query_extractor_error::QueryExtractorErrorNamed::from(err);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error);
        }
    };
    println!(
        "get query_parameters limit {:?}, name {:?} color {:?} select {}",
        query_parameters.limit,
        query_parameters.name,
        query_parameters.color,
        query_parameters.select
    );
    let limit = match &query_parameters.limit {
        Some(limit) => limit,
        None => &tufa_common::server::postgres::constants::DEFAULT_POSTGRES_SELECT_LIMIT,
    };
    let query_result = match (
        &query_parameters.name,
        &query_parameters.color,
        &query_parameters.select,
    ) {
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
                "SELECT id FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        }
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        }
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatColor,
                "SELECT color FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdName,
                "SELECT id, name FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatIdColor,
                "SELECT id, color FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatNameColor,
                "SELECT name, color FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "SELECT id, name, color FROM cats LIMIT $1",
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            None,
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE color = $1 LIMIT $2",
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            None,
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 LIMIT $2",
                name,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Name,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Color,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdName,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::NameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
        (
            Some(name),
            Some(color),
            tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::IdNameColor,
        ) => match sqlx::query_as!(
            tufa_common::repositories_types::tufa_server::routes::api::cats::CatName,
                "SELECT name FROM cats WHERE name = $1 AND color = $2 LIMIT $3",
                name,
                color,
                *limit as i64
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        {
            Ok(value) => Ok(value.into_iter()
                .map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element))
                .collect::<Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions>>()),
            Err(e) => Err(e)
        },
    };
    match query_result {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::Desirable(
            value.into_iter().map(|value_element| tufa_common::repositories_types::tufa_server::routes::api::cats::CatOptions::from(value_element)).collect()
        ),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGet::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get::TryGetResponseVariants::from(error)
        }
    }
}
