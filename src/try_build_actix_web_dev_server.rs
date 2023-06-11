// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for SayHi
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> actix_web::dev::Service<actix_web::dev::ServiceRequest> for SayHiMiddleware<S>
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future =
        futures_util::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

//todo - make it async trait after async trait stabilization
pub async fn try_build_actix_web_dev_server<'a>(
    tcp_listener: std::net::TcpListener,
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    redis_session_storage: actix_session::storage::RedisSessionStore,
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config
) -> Result<actix_web::dev::Server, Box<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer<'a>>>{
    // Shared Mutable State
    // use actix_web::{web, App, HttpServer};
    // use std::sync::Mutex;

    // struct AppStateWithCounter {
    //     counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
    // }

    // async fn index(data: web::Data<AppStateWithCounter>) -> String {
    //     let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    //     *counter += 1; // <- access counter inside MutexGuard

    //     format!("Request number: {counter}") // <- response with count
    // }

    // use actix_web::{web, App, HttpResponse, HttpServer};

    // // this function could be located in a different module
    // fn scoped_config(cfg: &mut web::ServiceConfig) {
    //     cfg.service(
    //         web::resource("/test")
    //             .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
    //             .route(web::head().to(HttpResponse::MethodNotAllowed)),
    //     );
    // }

    // // this function could be located in a different module
    // fn config(cfg: &mut web::ServiceConfig) {
    //     cfg.service(
    //         web::resource("/app")
    //             .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
    //             .route(web::head().to(HttpResponse::MethodNotAllowed)),
    //     );
    // }

    // #[actix_web::main]
    // async fn main() -> std::io::Result<()> {
    //     HttpServer::new(|| {
    //         App::new()
    //             .configure(config)
    //             .service(web::scope("/api").configure(scoped_config))
    //             .route(
    //                 "/",
    //                 web::get().to(|| async { HttpResponse::Ok().body("/") }),
    //             )
    //     })
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await
    // }

    let server = match actix_web::HttpServer::new(move || {
        let secret_key = actix_web::cookie::Key::from({
            use secrecy::ExposeSecret;
            use tufa_common::common::config::config_fields::GetHmacSecret;
            config.get_hmac_secret().expose_secret()
        }.as_bytes());
        actix_web::App::new()
        //todo - why no compile time error happens if you use 
        .wrap(actix_web_flash_messages::FlashMessagesFramework::builder(
            actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build()
            )
            .build()
        )
        .wrap(actix_session::SessionMiddleware::new(
            redis_session_storage.clone(),
            secret_key,
        ))
        .wrap(tracing_actix_web::TracingLogger::default())
        .wrap(
            actix_cors::Cors::default()
                .supports_credentials()
                // .allowed_origin(&config.get_access_control_allow_origin())
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .expose_any_header()
                .max_age({
                use tufa_common::common::config::config_fields::GetAccessControlMaxAge;
                *config.get_access_control_max_age()
                }),
        )
        // .wrap(SayHiMiddleware) 
        // .wrap_fn(|req, srv| {
        //     println!("Hi from start. You requested: {}", req.path());
        //     //
        //     match req.request().headers().get(tufa_common::common::git::project_git_info::PROJECT_COMMIT) {
        //         Some(project_commit_header_value) => match project_commit_header_value.to_str() {
        //             Ok(possible_project_commit) => {
        //                 println!("possible_project_commit {possible_project_commit}");
        //                 println!("PROJECT_GIT_INFO.project_commit {}", tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit);
        //                 if let true = possible_project_commit != tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit {
        //                     println!("1");
        //                     // let error = tufa_common::repositories_types::tufa_server::routes::api::cats::PatchErrorNamed::CheckApiUsage {
        //                     //     project_commit: app_info.project_git_info.does_not_match_message(),
        //                     //     code_occurence: tufa_common::code_occurence!(),
        //                     // };
        //                     // use tufa_common::common::error_logs_logic::error_log::ErrorLog;
        //                     // error.error_log(app_info.config);
        //                     // return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
        //                     //     error.into_serialize_deserialize_version()
        //                     // ));
        //                 }
        //                 else {
        //                     println!("1.5");
        //                 }
        //             },
        //             Err(e) => {
        //                 println!("2");
        //                 // let error = tufa_common::repositories_types::tufa_server::routes::api::cats::PatchErrorNamed::CannotConvertProjectCommitToStr {
        //                 //     cannot_convert_project_commit_to_str: format!("{}, error: {e}", app_info.project_git_info.cannot_convert_project_commit_to_str_message()),
        //                 //     code_occurence: tufa_common::code_occurence!(),
        //                 // };
        //                 // use tufa_common::common::error_logs_logic::error_log::ErrorLog;
        //                 // error.error_log(app_info.config);
        //                 // return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
        //                 //     error.into_serialize_deserialize_version()
        //                 // ));
        //             }
        //         },
        //         None => {
        //             println!("3");
        //             // let error = tufa_common::repositories_types::tufa_server::routes::api::cats::PatchErrorNamed::NoProjectCommitHeader {
        //             //     no_project_commit_header: app_info.project_git_info.no_project_commit_header_message(),
        //             //     code_occurence: tufa_common::code_occurence!(),
        //             // };
        //             // use tufa_common::common::error_logs_logic::error_log::ErrorLog;
        //             // error.error_log(app_info.config);
        //             // return actix_web::HttpResponse::BadRequest().json(actix_web::web::Json(
        //             //     error.into_serialize_deserialize_version()
        //             // ));
        //         }
        //     };
        //     //
        //     // return ErrorUnauthorized("ErrorUnauthorized");
        //     use actix_web::dev::Service;
        //     use futures_util::FutureExt;
        //     srv.call(req).map(|res| {
        //         // println!("Hi from response");
        //         res
        //         // actix_web::HttpResponse::BadRequest().finish()
        //     })
        // })
        //todo concrete host \ domain
        //
        .app_data(actix_web::web::Data::new({
            use tufa_common::common::config::get_email_client::GetEmailClient;
            config.get_email_client()
        }))
        .app_data(actix_web::web::Data::new({
            use tufa_common::common::config::config_fields::GetHmacSecret;
            config.get_hmac_secret().clone()
        }))
        .app_data(actix_web::web::Data::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo {
            postgres_pool: postgres_pool.clone(),//if use it without .clone() - will be runtime error if you try to reach route
            config: config,
            project_git_info: &tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO,
            repository_git_info: &crate::global_variables::compile_time::git_info::GIT_INFO,
        }))
        //todo - service capabilities ?
        .service(
            actix_web::web::scope("/admin")
                .guard(actix_web::guard::Host("127.0.0.1"))
                .wrap(actix_web_lab::middleware::from_fn(tufa_common::repositories_types::tufa_server::authentication::reject_anonymous_users))
                .route("/dashboard", actix_web::web::get().to(crate::routes::dashboard::admin_dashboard))
                // .route("/newsletters", web::get().to(tufa_common::repositories_types::tufa_server::routes::publish_newsletter_form))
                .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
                .route("/password", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::admin::change_password_form))
                .route("/password", actix_web::web::post().to(crate::routes::admin::password::change_password))
                .route("/logout", actix_web::web::post().to(tufa_common::repositories_types::tufa_server::routes::admin::log_out)),
        )
        .route("/login", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::login::login_form))
        .route("/login", actix_web::web::post().to(crate::routes::login::login))
        .route("/subscriptions", actix_web::web::post().to(crate::routes::subscribe))
        .route("/subscriptions/confirm", actix_web::web::get().to(crate::routes::confirm))
        .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
        //
        .route("/health_check", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::health_check))
        .service(
            actix_web::web::scope("/api")
            .service(tufa_common::server::routes::git_info::git_info)
            .service(
            // actix_web::web::resource("/cats")
                actix_web::web::scope(&format!("/{}", tufa_common::repositories_types::tufa_server::routes::api::cats::CATS))
                // .guard(actix_web::guard::Host("www.rust-lang.org"))
                .service(crate::routes::api::cats::get)
                .service(crate::routes::api::cats::get_by_id)
                .service(crate::routes::api::cats::post)
                .service(crate::routes::api::cats::put)
                .service(crate::routes::api::cats::patch)
                .service(crate::routes::api::cats::delete)
                .service(crate::routes::api::cats::delete_by_id)
            )
        )


    })
    .listen(tcp_listener)
    {
        Ok(server) => server,
        Err(e) => {
            return Err(Box::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer::HttpServerListen {
                http_server_listen: e,
                code_occurence: tufa_common::code_occurence!(),
            }))
        }
    }
    .run();
    Ok(server)
}
