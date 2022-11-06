use crate::project_constants::PROJECT_NAME;
use lazy_static::lazy_static;
use tufa_common::common::git::git_info_wrapper::GitInformationWrapper;

lazy_static! {
    pub static ref GIT_INFO: GitInformationWrapper =
        GitInformationWrapper::init("../", PROJECT_NAME);
}
