// #[derive(Debug, thiserror::Error)]
// pub enum OneError {
//     One(String)
// }
// impl std::fmt::Display for OneError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn one() -> Result<(), OneError> {
//     Err(OneError::One(String::from("one")))
// }
// #[derive(Debug, thiserror::Error)]
// pub enum TwoError {
//     Two(String)
// }
// impl std::fmt::Display for TwoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn two() -> Result<(), TwoError> {
//     Err(TwoError::Two(String::from("two")))
// }
// #[derive(Debug, thiserror::Error)]
// pub enum ThreeError {
//     One {
//         #[source]
//         one: OneError,
//     },
//     Two {
//         #[source]
//         two: TwoError,
//     },
//     OneTwo {
//         #[source]
//         one: OneError,
//         // #[source] //error: duplicate #[source] attribute - и что с этим делать?
//         two: TwoError,
//     }
// }
// impl std::fmt::Display for ThreeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn three() -> Result<(), ThreeError> {
//     match futures::join!(
//         one(),
//         two()
//     ) {
//         (Ok(_), Ok(_)) => Ok(()),
//         (Err(one), Ok(_)) => Err(ThreeError::One {
//             one,
//         }),
//         (Ok(_), Err(two)) => Err(ThreeError::Two {
//             two,
//         }),
//         (Err(one), Err(two)) => Err(ThreeError::OneTwo {
//             //что с этим делать?
//             one,
//             two,
//         }),
//     }
// }
// // pub trait Error: Debug + Display {
// //     fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
// //     fn description(&self) -> &str { ... }
// //     fn cause(&self) -> Option<&dyn Error> { ... }
// //     fn provide<'a>(&'a self, demand: &mut Demand<'a>) { ... }
// // }

pub async fn dev() {
    // let app = axum::Router::new().route(
    //     "/kekw",
    //     axum::routing::get(|| async {
    //         println!("handler");
    //         "Hello, World!"
    //     }),
    // );

    // // run it with hyper on localhost:3000
    // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    let app = Router::new()
        .route("/", get(handler))
        .route_layer(middleware::from_fn(auth));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[derive(Clone)]
struct CurrentUser {/* ... */}

async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    println!("authmiddleware");
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    println!("{auth_header:#?}");
    let auth_header = if let Some(auth_header) = auth_header {
        println!("1");
        auth_header
    } else {
        println!("2");
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    // ...
    None
}

async fn handler(
    // extract the current user, set by the middleware
    Extension(current_user): Extension<CurrentUser>,
) {
    println!("handler");
    // ...
}

// impl actix_web::FromRequest for ProjectCommitExtractor {
//     type Error = actix_web::Error;
//     type Future = std::future::Ready<Result<Self, Self::Error>>;
//     fn from_request(
//         req: &actix_web::HttpRequest,
//         _payload: &mut actix_web::dev::Payload,
//     ) -> Self::Future {
//         match req
//             .headers()
//             .get(crate::common::git::project_git_info::PROJECT_COMMIT)
//         {
//             Some(project_commit_header_value) => {
//                 match project_commit_header_value.to_str() {
//                     Ok(possible_project_commit) => {
//                         match possible_project_commit
//                         == crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
//                             .project_commit
//                     {
//                         true => std::future::ready(Ok(ProjectCommitExtractor {})),
//                         false => {
//                             std::future::ready(Err({
//                                 let error_with_serialize_deserialize = ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual {
//                                     project_commit_not_equal: "different project commit provided, services must work only with equal project commits",
//                                     project_commit_to_use: {
//                                         use crate::common::git::get_git_commit_link::GetGitCommitLink;
//                                         crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.get_git_commit_link()
//                                     },
//                                     code_occurence: crate::code_occurence_tufa_common!(),
//                                 }.into_serialize_deserialize_version();
//                                 actix_web::error::ErrorBadRequest(actix_web::web::Json(serde_json::to_string(&error_with_serialize_deserialize).unwrap_or_else(|_|{
//                                     crate::common::to_default_stringified_json::ToDefaultStringifiedJson::to_default_stringified_json(&error_with_serialize_deserialize)
//                                 })))
//                             }))
//                         }
//                     }
//                     }
//                     Err(e) => std::future::ready(Err({
//                         let error_with_serialize_deserialize = ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorToStrConversion {
//                             project_commit_to_str_conversion: e,
//                             code_occurence: crate::code_occurence_tufa_common!()
//                         }.into_serialize_deserialize_version();
//                         actix_web::error::ErrorBadRequest(actix_web::web::Json(serde_json::to_string(&error_with_serialize_deserialize).unwrap_or_else(|_|{
//                             crate::common::to_default_stringified_json::ToDefaultStringifiedJson::to_default_stringified_json(&error_with_serialize_deserialize)
//                         })))
//                     })),
//                 }
//             }
//             None => std::future::ready(Err({
//                 let error_with_serialize_deserialize = ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader {
//                     no_project_commit_header: "project_commit header is not provided",
//                     code_occurence: crate::code_occurence_tufa_common!()
//                 }.into_serialize_deserialize_version();
//                 actix_web::error::ErrorBadRequest(actix_web::web::Json(serde_json::to_string(&error_with_serialize_deserialize).unwrap_or_else(|_|{
//                     crate::common::to_default_stringified_json::ToDefaultStringifiedJson::to_default_stringified_json(&error_with_serialize_deserialize)
//                 })))
//             })),
//         }
//     }
// }
