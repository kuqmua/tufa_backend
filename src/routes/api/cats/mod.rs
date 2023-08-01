mod delete;
mod delete_by_id;
mod get;
mod get_by_id;
mod patch;
mod post;
mod put;

//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
//todo - add limit everywhere possible
//todo header Retry-After logic
//todo - its the case if all columns except id are not null. for nullable columns must be different logic(post or put)

pub(crate) fn routes(
    app_info: tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new()
        .route(
            &format!(
                "/{}/",
                tufa_common::repositories_types::tufa_server::routes::api::cats::CATS
            ),
            axum::routing::get(crate::routes::api::cats::get::get)
                .post(crate::routes::api::cats::post::post)
                .put(crate::routes::api::cats::put::put)
                .patch(crate::routes::api::cats::patch::patch)
                .delete(crate::routes::api::cats::delete::delete),
        )
        .route(
            &format!(
                "/{}/:id",
                tufa_common::repositories_types::tufa_server::routes::api::cats::CATS
            ),
            axum::routing::get(crate::routes::api::cats::get_by_id::get_by_id)
                .delete(crate::routes::api::cats::delete_by_id::delete_by_id),
        )
        .layer(tower_http::cors::CorsLayer::new().allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PATCH,
            http::Method::PUT,
            http::Method::DELETE,
        ]))
        .route_layer(axum::middleware::from_fn_with_state(
            app_info.clone(),
            tufa_common::server::middleware::project_commit_checker::project_commit_checker,
        ))
        .with_state(app_info)
}
