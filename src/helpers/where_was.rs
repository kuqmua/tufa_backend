use crate::helpers::get_git_source_file_link::get_git_source_file_link;
use crate::traits::where_was_trait::WhereWasTrait;

#[derive(Debug)]
pub struct WhereWas {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl WhereWasTrait for WhereWas {
    fn source_place(&self) -> String {
        format!("{}{}{}", self.file, self.line, self.column)
    }
    fn github_source_place(&self) -> String {
        get_git_source_file_link(self.file, self.line)
    }
}
