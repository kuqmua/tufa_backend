//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
//todo - add limit everywhere possible
//todo header Retry-After logic
//todo - its the case if all columns except id are not null. for nullable columns must be different logic(post or put)

pub fn routes(
    app_info: tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new().nest(
        &format!(
            "/{}",
            tufa_common::repositories_types::tufa_server::routes::api::cats::TABLE_NAME
        ),
        axum::Router::new().merge(crud(app_info)),
    )
}

fn crud(
    app_info: tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new()
        .route(
            "/search",
            axum::routing::post(
                tufa_common::repositories_types::tufa_server::routes::api::cats::read_with_body,
            )
            .delete(
                tufa_common::repositories_types::tufa_server::routes::api::cats::delete_with_body,
            ),
        )
        .route(
            "/batch",
            axum::routing::post(
                tufa_common::repositories_types::tufa_server::routes::api::cats::create_batch,
            )
            .patch(tufa_common::repositories_types::tufa_server::routes::api::cats::update),
        )
        .route(
            "/",
            axum::routing::get(
                tufa_common::repositories_types::tufa_server::routes::api::cats::read,
            )
            .post(tufa_common::repositories_types::tufa_server::routes::api::cats::create)
            .delete(tufa_common::repositories_types::tufa_server::routes::api::cats::delete),
        )
        .route(
            "/:id",
            axum::routing::get(
                tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id,
            )
            .patch(tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id)
            .delete(tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id),
        )
        .layer(tower_http::cors::CorsLayer::new().allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PATCH,
            http::Method::DELETE,
        ]))
        .route_layer(axum::middleware::from_fn_with_state(
            app_info.clone(),
            tufa_common::server::middleware::project_commit_checker::project_commit_checker,
        ))
        .with_state(app_info)
}
