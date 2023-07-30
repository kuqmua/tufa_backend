pub(crate) async fn get_by_id_axum(
    axum::extract::Path(path_parameters): axum::extract::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::GetByIdPathParameters>,
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error);
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
    )
    .fetch_one(&*app_info.get_postgres_pool())
    .await
    {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::DesirableType(value),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error)
        }
    }
}