use crate::lazy_static::git_info::GIT_INFO;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn git_info_html() -> HttpResponse {
    {
        HttpResponse::Ok().content_type(ContentType::html()).body(
            GIT_INFO
                .data
                .get_git_html_info(GIT_INFO.data.get_commit_link()),
        )
    }
}
