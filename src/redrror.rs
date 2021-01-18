mod override_prints {
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
}
