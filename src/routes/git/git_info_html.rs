use crate::global_variables::compile_time::git_info::GIT_INFO;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use tufa_common::traits::get_git_html_info::GetGitHtmlInfo;

pub async fn git_info_html() -> HttpResponse {
    {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(GIT_INFO.get_git_html_info())
    }
}
