use crate::project_constants::PROJECT_NAME;
use once_cell::sync::Lazy;
use tufa_common::common::git::git_info_wrapper::GitInformationWrapper;

pub static GIT_INFO: Lazy<GitInformationWrapper> =
    Lazy::new(|| GitInformationWrapper::init("../", PROJECT_NAME));
