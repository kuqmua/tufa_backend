use crate::project_constants::PROJECT_NAME;
use once_cell::sync::Lazy;
use tufa_common::common::git::git_info::GitInformation;

pub static GIT_INFO: Lazy<GitInformation> =
    Lazy::new(|| GitInformation::get_git_commit_info("../", PROJECT_NAME));
