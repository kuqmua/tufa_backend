pub(crate) async fn patch_by_id<'a>(
    path_parameters_extraction_result: Result<
        axum::extract::Path<
            tufa_common::repositories_types::tufa_server::routes::api::cats::PatchByIdPathParameters,
        >,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let path_parameters = match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::PatchByIdPathParameters,
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants
    >::try_extract_value(
        path_parameters_extraction_result,
        &app_info_state
    ) {
        Ok(path_parameters) => path_parameters,
        Err(err) => {
            return err;
        },
    };
    println!("patch_by_id path_parameters {path_parameters:#?}");
    let payload = match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch,
        tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants
    >::try_extract_value(
        payload_extraction_result,
        &app_info_state
    ) {
        Ok(payload) => payload,
        Err(err) => {
            return err;
        },
    };
    println!("patch payload {payload:#?}");
    let query_result = match payload {
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::Name { name } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1 WHERE id = $2",
                name,
                path_parameters.id.to_inner()
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::Color { color } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET color = $1 WHERE id = $2",
                color,
                path_parameters.id.to_inner()
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::NameColor { name, color } => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "UPDATE cats SET name = $1, color = $2 WHERE id = $3",
                name,
                color,
                path_parameters.id.to_inner()
            )
            .fetch_all(&*app_info_state.get_postgres_pool())
            .await
        },
    };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants::Desirable(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                app_info_state.as_ref(),
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::patch_by_id::TryPatchByIdResponseVariants::from(error)
        }
    }
}
