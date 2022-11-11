use crate::project_constants::PROJECT_NAME;
use compile_time_git_info::CompileTimeGitInfoTufaServer;
use once_cell::sync::Lazy;
use tufa_common::common::git::git_info::GitInformation;

#[derive(Debug, CompileTimeGitInfoTufaServer)]
pub struct GitInfoGlobalStaticConst {}
