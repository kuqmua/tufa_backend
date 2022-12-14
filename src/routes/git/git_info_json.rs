use crate::global_variables::compile_time::git_info::GIT_INFO;
use actix_web::{web, Responder};
use tufa_common::common::git::git_info::GitInformation;

pub async fn git_info_json() -> impl Responder {
    web::Json(GitInformation {
        git_commit_id: GIT_INFO.git_commit_id,
        git_repo_link: GIT_INFO.git_repo_link,
        git_author: GIT_INFO.git_author,
        git_author_email: GIT_INFO.git_author_email,
        git_commit_unix_time: GIT_INFO.git_commit_unix_time,
        git_timezone: GIT_INFO.git_timezone,
        git_message: GIT_INFO.git_message,
    })
}
