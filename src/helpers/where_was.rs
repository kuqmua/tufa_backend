use crate::helpers::get_git_source_file_link::get_git_source_file_link;
use crate::traits::where_was_trait::WhereWasTrait;

pub struct WhereWas {
    file: &'static str,
    line: u32,
    column: u32,
}

impl WhereWasTrait for WhereWas {
    fn source_place(&self) -> String {
        format!("{}{}{}", self.file, self.line, self.column)
    }
    fn github_source_place(&self) -> String {
        get_git_source_file_link(self.file, self.line)
    }
}
