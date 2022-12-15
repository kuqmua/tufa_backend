use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use actix_web::{web, Responder};
use tufa_common::common::git::git_info::GitInformation;

pub async fn git_info_json() -> impl Responder {
    web::Json(GitInformation {
        git_commit_id: &GIT_INFO_WITHOUT_LIFETIMES.git_commit_id,
        git_repo_link: &GIT_INFO_WITHOUT_LIFETIMES.git_repo_link,
        git_author: &GIT_INFO_WITHOUT_LIFETIMES.git_author,
        git_author_email: &GIT_INFO_WITHOUT_LIFETIMES.git_author_email,
        git_commit_unix_time: &GIT_INFO_WITHOUT_LIFETIMES.git_commit_unix_time,
        git_timezone: &GIT_INFO_WITHOUT_LIFETIMES.git_timezone,
        git_message: &GIT_INFO_WITHOUT_LIFETIMES.git_message,
    })
}
