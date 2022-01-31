use crate::helpers::get_git_info::{get_git_info, GitCommitInfo};

lazy_static! {
    pub static ref GIT_INFO: GitCommitInfo = get_git_info();
}
