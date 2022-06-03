use crate::helpers::git_info::GIT_INFO;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use tufa_common::helpers::git::get_git_html_info::get_git_html_info;

pub async fn git_info_html() -> HttpResponse {
    {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(get_git_html_info(
                GIT_INFO.commit_id.clone(),
                GIT_INFO.repo_link.clone(),
                GIT_INFO.author.clone(),
                GIT_INFO.author_email.clone(),
                GIT_INFO.commit_unix_time.clone(),
                GIT_INFO.timezone.clone(),
                GIT_INFO.message.clone(),
                GIT_INFO.get_commit_link(),
            ))
    }
}
