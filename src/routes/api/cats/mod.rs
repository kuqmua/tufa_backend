mod create;
mod create_batch;
mod create_or_update;
mod create_or_update_by_id;
mod delete;
mod delete_by_id;
mod delete_post;
mod read;
mod read_by_id;
mod read_post;
mod update;
mod update_by_id;

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
            tufa_common::repositories_types::tufa_server::routes::api::cats::CATS
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
            axum::routing::post(crate::routes::api::cats::read_post::read_post)
                .delete(crate::routes::api::cats::delete_post::delete_post),
        )
        .route(
            "/batch",
            axum::routing::post(crate::routes::api::cats::create_batch::create_batch)
                .patch(crate::routes::api::cats::update::update)
                .put(crate::routes::api::cats::create_or_update::create_or_update),
        )
        .route(
            "/",
            axum::routing::get(crate::routes::api::cats::read::read)
                .post(crate::routes::api::cats::create::create)
                .delete(crate::routes::api::cats::delete::delete),
        )
        .route(
            "/:id",
            axum::routing::get(crate::routes::api::cats::read_by_id::read_by_id)
                .put(crate::routes::api::cats::create_or_update_by_id::create_or_update_by_id)
                .patch(crate::routes::api::cats::update_by_id::update_by_id)
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
