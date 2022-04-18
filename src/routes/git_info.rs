use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use tufa_common::helpers::git::lazy_static_git_info::GIT_INFO;
use tufa_common::helpers::git::get_git_commit_string::get_git_commit_string;

pub struct GitCommitInfo {
    pub commit_message: String,
    pub commit_id: String,
    pub branch: String,
    pub repo_link: String,
}
pub async fn git_info() -> HttpResponse {
    let commit_message = GIT_INFO.commit_message.clone();
    let commit_id = GIT_INFO.commit_id.clone();
    let branch = GIT_INFO.branch.clone();
    let repo_link = GIT_INFO.repo_link.clone();
    let commit_string = get_git_commit_string();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
<div style="display:flex; justify-content: center; align-items: center; flex-direction: column; width: 100%; height: 100%;">
<div>Commit message: {commit_message}</div>
<div>Commit id: {commit_id}</div>
<div>Branch: {branch}</div>
<div>Repo link: {repo_link}</div>
<div>Commit string: {commit_string}</div>
</div>
    
</body>
</html>"#,
        ))
}
