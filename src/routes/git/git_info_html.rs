use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use tufa_common::traits::get_git_html_info::GetGitHtmlInfo;

pub async fn git_info_html() -> HttpResponse {
    {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(GIT_INFO_WITHOUT_LIFETIMES.get_git_html_info())
    }
}
