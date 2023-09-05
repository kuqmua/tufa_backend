pub(crate) async fn update_by_id<'a>(
    //todo how to check with type system what http request function params and route path query and payload params are same?
    path_parameters_extraction_result: Result<
        axum::extract::Path<
            tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPath,
        >,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<
            tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPayload,
        >,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters =
        tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdParameters {
            path: match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
                tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPath,
                tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants
            >::try_extract_value(
                path_parameters_extraction_result,
                &app_info_state
            ) {
                Ok(path_parameters) => path_parameters,
                Err(err) => {
                    return err;
                },
            },
            payload: match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPayload,
                tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants
            >::try_extract_value(
                payload_extraction_result,
                &app_info_state
            ) {
                Ok(payload) => payload,
                Err(err) => {
                    return err;
                },
            },
        };
    println!("{parameters:#?}");
    let query_result = match (parameters.payload.name, parameters.payload.color) {
        (None, None) => {
            return tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no_payload_fields"),
                code_occurence: tufa_common::code_occurence!(),
            };
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET color = $1 WHERE id = $2",
                color,
                parameters.path.id.to_inner()
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                parameters.path.id.to_inner()
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1, color = $2 WHERE id = $3",
                name,
                color,
                parameters.path.id.to_inner()
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        }
    };
    // let query_result = match parameters.payload {
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPayload::Name { name } => {
    //         sqlx::query_as!(
    //             tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
    //             "UPDATE cats SET name = $1 WHERE id = $2",
    //             name,
    //             parameters.path.id.to_inner()
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     },
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPayload::Color { color } => {
    //         sqlx::query_as!(
    //             tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
    //             "UPDATE cats SET color = $1 WHERE id = $2",
    //             color,
    //             parameters.path.id.to_inner()
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     },
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPayload::NameColor { name, color } => {
    //         sqlx::query_as!(
    //             tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
    //             "UPDATE cats SET name = $1, color = $2 WHERE id = $3",
    //             name,
    //             color,
    //             parameters.path.id.to_inner()
    //         )
    //         .fetch_all(&*app_info_state.get_postgres_pool())
    //         .await
    //     },
    // };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateByIdResponseVariants::from(error)
        }
    }
}
