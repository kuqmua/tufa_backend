// #[derive(utoipa::OpenApi)]
// #[openapi(
//     paths(
//         tufa_common::server::routes::git_info::git_info,
//     ),
//     components(
//         schemas(tufa_common::server::routes::git_info::Todo)
//     ),
//     modifiers(&SecurityAddon),
//     tags(
//         (name = "todo", description = "Todo items management API")
//     )
// )]
// struct ApiDoc;

// struct SecurityAddon;
// impl utoipa::Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         if let Some(components) = openapi.components.as_mut() {
//             components.add_security_scheme(
//                 "api_key",
//                 utoipa::openapi::security::SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new("todo_apikey"))),
//             )
//         }
//     }
// }

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
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::try_build_server::TryBuildServer>>
{
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
    ); //: std::sync::Arc<dyn tufa_common::repositories_types::tufa_server::routes::service_possibilities::ServicePossibilities + Send + Sync>
    let shared_data = SharedData {
        message: std::string::String::from("shared_message"),
    };
    let listener = TcpListener::bind("127.0.0.1").await.unwrap();
    // axum::Server
    // ::bind(
    //     tufa_common::common::config::config_fields::GetSocketAddr::get_socket_addr(config),
    // )
    axum::serve(
        listener,
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
                        http::Method::DELETE,
                    ])
                    .allow_origin(["http://127.0.0.1".parse().unwrap()]),
            )
            // .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url(
            //     "/api-docs/openapi.json",
            //     {
            //         use utoipa::OpenApi;
            //         ApiDoc::openapi()
            //     },
            // ))
            .into_make_service(),
    )
    .await
    .unwrap_or_else(|e| panic!("axum builder serve await failed {e:#?}"));
    Ok(())
}

///////////////////////////////////
// this     works

// pub struct One(String);
// pub struct Two(String);
// pub struct AppInfo(One, Two);
// pub trait GetOne {
//     fn get_one(&self) -> &One;
// }
// pub trait GetTwo {
//     fn get_two(&self) -> &Two;
// }
// pub trait GetOneGetTwo: GetOne + GetTwo {}
// impl GetOne for AppInfo {
//     fn get_one(&self) -> &One {
//         &self.0
//     }
// }
// impl GetTwo for AppInfo {
//     fn get_two(&self) -> &Two {
//         &self.1
//     }
// }
// impl GetOneGetTwo for AppInfo {}
// pub struct Example {}
// pub trait DoSomething {
//     fn do_something<T: GetOne + ?Sized>(&self, handle_get_one: &T);
// }
// impl DoSomething for Example {
//     fn do_something<T: GetOne + ?Sized>(&self, handle_get_one: &T) {
//         println!("{}", handle_get_one.get_one().0);
//     }
// }
// pub async fn something(
//     app_info_state: axum::extract::State<std::sync::Arc<dyn GetOneGetTwo + Send + Sync>>,
// ) {
//     let example = Example {};
//     example.do_something(app_info_state.as_ref());
// }
// #[tokio::main]
// async fn main() {
//     //case1 - compiles
//     // let router = axum::Router::new()
//     //     .route("/", axum::routing::get(something))
//     //     .with_state(Box::new(std::sync::Arc::new(AppInfo(
//     //         One(String::from("one")),
//     //         Two(String::from("two")),
//     //     ))))
//     //     .into_make_service();
//     //case2
//     let app_info: std::sync::Arc<dyn GetOneGetTwo + Send + Sync> =
//         std::sync::Arc::new(AppInfo(One(String::from("one")), Two(String::from("two"))));
//     let router = axum::Router::new()
//         .route("/", axum::routing::get(something))
//         // expected struct `std::boxed::Box<std::sync::Arc<(dyn GetOneGetTwo + std::marker::Send + std::marker::Sync + 'static)>>`
//         // found struct `std::boxed::Box<std::sync::Arc<AppInfo>>`
//         .with_state(app_info)
//         .into_make_service();
//     //
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(router)
//         .await
//         .unwrap();
// }




//
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     #[derive(utoipa::OpenApi)]
//     #[openapi(
//         paths(
//             todo::list_todos,
//         ),
//         components(
//             schemas(todo::Todo, )//todo::TodoError
//         ),
//         modifiers(&SecurityAddon),
//         tags(
//             (name = "todo", description = "Todo items management API")
//         )
//     )]
//     struct ApiDoc;
//     struct SecurityAddon;
//     impl utoipa::Modify for SecurityAddon {
//         fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//             if let Some(components) = openapi.components.as_mut() {
//                 components.add_security_scheme(
//                     "api_key",
//                     utoipa::openapi::security::SecurityScheme::ApiKey(
//                         utoipa::openapi::security::ApiKey::Header(
//                             utoipa::openapi::security::ApiKeyValue::new("todo_apikey"),
//                         ),
//                     ),
//                 )
//             }
//         }
//     }
//     let store = std::sync::Arc::new(crate::todo::Store::default());
//     let app = axum::Router::new()
//         .merge(
//             utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", {
//                 use utoipa::OpenApi;
//                 ApiDoc::openapi()
//             }),
//         )
//         .route("/todo", axum::routing::get(todo::list_todos))
//         .with_state(store);

//     let address = std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, 8080));
//     let listener = tokio::net::TcpListener::bind(&address).await?;
//     axum::serve(listener, app.into_make_service()).await
// }
// mod todo {
//     pub type Store = tokio::sync::Mutex<Vec<Todo>>;
//     #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
//     pub struct Todo {
//         id: i32,
//         #[schema(example = "Buy groceries")]
//         value: String,
//         done: bool,
//     }
//     // #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
//     // pub enum TodoError {
//     //     #[schema(example = "Todo already exists")]
//     //     Conflict(String),
//     //     #[schema(example = "id = 1")]
//     //     NotFound(String),
//     //     #[schema(example = "missing api key")]
//     //     Unauthorized(String),
//     // }
//     #[utoipa::path(
//         get,
//         path = "/todo",
//         responses(
//             (status = 200, description = "List all todos successfully", body = [Todo])
//         )
//     )]
//     pub async fn list_todos(
//         axum::extract::State(store): axum::extract::State<std::sync::Arc<Store>>,
//     ) -> axum::Json<Vec<Todo>> {
//         let todos = store.lock().await.clone();
//         axum::Json(todos)
//     }
// }
//