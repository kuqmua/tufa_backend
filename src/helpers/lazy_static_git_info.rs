use crate::helpers::get_git_info::get_git_info;

lazy_static! {
    pub static ref GIT_INFO: String = get_git_info();
}
