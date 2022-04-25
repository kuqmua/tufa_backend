use actix_web::{web, Responder};
use crate::helpers::git_info::GIT_INFO;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitJsonInformation {
    pub commit_id: String,
    pub repo_link: String,
    pub author: String,
    pub author_email: String,
    pub commit_unix_time: String,
    pub timezone: String,
    pub message: String,
    pub commit_link: String,
}

pub async fn git_info_json() -> impl Responder {
    web::Json(GitJsonInformation {
        commit_id: GIT_INFO.commit_id.clone(),
        repo_link: GIT_INFO.repo_link.clone(),
        author: GIT_INFO.author.clone(),
        author_email: GIT_INFO.author_email.clone(),
        commit_unix_time: GIT_INFO.commit_unix_time.clone(),
        timezone: GIT_INFO.timezone.clone(),
        message: GIT_INFO.message.clone(),
        commit_link: GIT_INFO.get_commit_link(),
    })
}