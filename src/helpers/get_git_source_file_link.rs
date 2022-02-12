use super::lazy_static_git_info::GIT_INFO;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_git_source_file_link(file: String, line: u32) -> String {
    format!(
        "https://{}/blob/{}/{}#L{}",
        GIT_INFO.repo_link, GIT_INFO.commit_id, file, line
    )
}
