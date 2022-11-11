use crate::once_cell_globals::git_info::GIT_INFO;
use actix_web::{web, Responder};
use tufa_common::common::git::git_info::GitInformation;

pub async fn git_info_json() -> impl Responder {
    web::Json(GitInformation {
        commit_id: GIT_INFO.commit_id,
        repo_link: GIT_INFO.repo_link,
        author: GIT_INFO.author,
        author_email: GIT_INFO.author_email,
        commit_unix_time: GIT_INFO.commit_unix_time,
        timezone: GIT_INFO.timezone,
        message: GIT_INFO.message,
    })
}
