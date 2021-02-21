use colored::*;
pub fn print_error_red(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        "file: ".red().bold(),
        file.red().bold(),
        ":".red().bold(),
        line.red().bold(),
        error.red().bold()
    );
}
pub fn print_warning_yellow(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        "file: ".yellow().bold(),
        file.yellow().bold(),
        ":".yellow().bold(),
        line.yellow().bold(),
        error.yellow().bold()
    );
}
