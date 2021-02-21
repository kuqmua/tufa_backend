use colored::*;
// use log::{debug, error, info, warn};
// use std::{fs::File, io::Write};
//handle error
pub fn print_error_red(file: String, line: String, error: String) {
    // debug!("something {}", error);
    // error!("waaarn {}", error);

    // println!("print error red, {}", error);
    // let mut fileonos = File::create("errorlogs.txt").expect("could not create file");
    // writeln!(&mut fileonos, "{}", error).unwrap();

    // let result_of_writing = fileonos.write(error.as_bytes());
    // match result_of_writing {
    //     Ok(_) => println!("записано"),
    //     Err(e) => println!("error {}", e),
    // }
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
