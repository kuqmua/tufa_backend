use crate::helpers::lazy_static_git_info::GIT_INFO;

//todo: make it only for Error impl
pub trait GitInfo {
    fn get_git_info(&self) -> &GIT_INFO; //return lazy static String (execute one time on program start)
}
