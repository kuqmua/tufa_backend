use crate::project_constants::PROJECT_NAME;
use lazy_static::lazy_static;
use tufa_common::helpers::git::git_info::GitInformation;

pub struct GitInformationWrapper {
    pub data: GitInformation,
}

impl GitInformationWrapper {
    pub fn init(repo_git_path: &str, repo_name: &str) -> Self {
        GitInformationWrapper {
            data: GitInformation::get_git_commit_info(repo_git_path, repo_name),
        }
    }
}

// lazy_static! {
//     pub static ref GIT_INFO: GitInformation =
//         GitInformation::get_git_commit_info("./", PROJECT_NAME);
// }

lazy_static! {
    pub static ref GIT_INFO: GitInformationWrapper =
        GitInformationWrapper::init("./", PROJECT_NAME);
}
