// // allow to open source files in browser like php
// fn routes_static() -> axum::Router {
//     axum::Router::new().nest_service(
//         "/",
//         axum::routing::get_service(tower_http::services::ServeDir::new("./")),
//     )
// }

async fn extract_custom_header_example(headers: http::header::HeaderMap) {
    let pc = headers.get("project_commit");
    println!("pc{pc:#?}")
}

async fn header_extractor_example(
    axum::TypedHeader(header): axum::TypedHeader<axum::headers::UserAgent>,
) {
    println!("header{:#?}", header);
}

async fn middleware_message_example(axum::Extension(shared_data): axum::Extension<SharedData>) {
    println!("message {}", shared_data.message);
}

#[derive(Clone)]
struct SharedData {
    pub message: std::string::String,
}

#[derive(Clone)] //or maybe add Clone to AppInfo too to solve possible problem?
struct HeaderMessage(pub std::string::String);

async fn read_middleware_custom_header(
    axum::Extension(message): axum::Extension<HeaderMessage>,
) -> std::string::String {
    println!("read_middleware_custom_header {}", message.0);
    message.0
}

pub async fn set_middleware_custom_header<B>(
    mut req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    let request_project_commit = req
        .headers()
        .get(tufa_common::common::git::project_git_info::PROJECT_COMMIT)
        .ok_or_else(|| axum::http::StatusCode::BAD_REQUEST)?;
    let project_commit_checker_header = request_project_commit
        .to_str()
        .map_err(|_error| axum::http::StatusCode::BAD_REQUEST)?
        .to_owned();
    let extensions = req.extensions_mut();
    extensions.insert(HeaderMessage(project_commit_checker_header.to_owned()));
    Ok(next.run(req).await)
}

//todo - make it async trait after async trait stabilization
pub async fn try_build_server<'a>(
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config,
) -> Result<
    (),
    Box<tufa_common::repositories_types::tufa_server::try_build_server::TryBuildServer<'a>>,
> {
    println!(
        "server running on {}",
        tufa_common::common::config::get_server_address::GetServerAddress::get_server_address(
            &config
        )
    );
    let app_info = std::sync::Arc::new(
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo {
            postgres_pool,
            config,
            project_git_info:
                &tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO,
            repository_git_info: &crate::global_variables::compile_time::git_info::GIT_INFO,
        },
    );
    let shared_data = SharedData {
        message: std::string::String::from("shared_message"),
    };
    axum::Server::bind(
        tufa_common::common::config::config_fields::GetSocketAddr::get_socket_addr(config),
    )
    .serve(
        axum::Router::new()
            .route(
                "/read_middleware_custom_header",
                axum::routing::get(read_middleware_custom_header),
            )
            .route(
                "/header_extractor_example",
                axum::routing::get(header_extractor_example),
            )
            .route(
                "/extract_custom_header_example",
                axum::routing::get(extract_custom_header_example),
            )
            .route_layer(axum::middleware::from_fn(set_middleware_custom_header))
            .route(
                "/middleware_message_example",
                axum::routing::get(middleware_message_example),
            )
            .layer(axum::Extension(shared_data))
            .merge(tufa_common::server::routes::routes(app_info.clone()))
            .merge(crate::routes::api::routes(app_info.clone()))
            .merge(tufa_common::server::routes::not_found::not_found_route(
                app_info.clone(),
            ))
            // .fallback_service(routes_static())
            .layer(
                tower_http::cors::CorsLayer::new()
                    .allow_methods([
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PATCH,
                        http::Method::PUT,
                        http::Method::DELETE,
                    ])
                    .allow_origin([
                        "http://127.0.0.1".parse().unwrap(),
                        "http://localhost".parse().unwrap(),
                    ]),
            )
            .into_make_service(),
    )
    .await
    .unwrap_or_else(|e| panic!("axum builder serve await failed {e:#?}"));
    Ok(())
}
