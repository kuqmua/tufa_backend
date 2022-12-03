use crate::global_variables::compile_time::git_info::GIT_INFO;
use once_cell::sync::Lazy;
use tufa_common::common::git::git_info::GitInformationWithoutLifetimes;

pub static GIT_INFO_WITHOUT_LIFETIMES: Lazy<GitInformationWithoutLifetimes> =
    Lazy::new(|| GIT_INFO.get_runtime_version());
