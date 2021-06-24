use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::RGB;
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
    //     Ok(_) => println!("written"),
    //     Err(e) => println!("error {}", e),
    // }
    eprintln!(
        "{}{}{}{}\n{}",
        Red.paint("file: "),
        Red.paint(file),
        Red.paint(":"),
        Red.paint(line),
        Red.bold().paint(error)
    );
}

pub fn print_warning_orange(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        RGB(255, 165, 0).paint("file: "),
        RGB(255, 165, 0).paint(file),
        RGB(255, 165, 0).paint(":"),
        RGB(255, 165, 0).paint(line),
        RGB(255, 165, 0).bold().paint(error)
    );
}

pub fn print_warning_yellow(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        Yellow.paint("file: "),
        Yellow.paint(file),
        Yellow.paint(":"),
        Yellow.paint(line),
        Yellow.bold().paint(error)
    );
}

pub fn print_success_green(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        RGB(0, 255, 0).paint("file: "),
        RGB(0, 255, 0).paint(file),
        RGB(0, 255, 0).paint(":"),
        RGB(0, 255, 0).paint(line),
        RGB(0, 255, 0).bold().paint(error)
    );
}

pub fn print_partial_success_cyan(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        RGB(0, 200, 155).paint("file: "),
        RGB(0, 200, 155).paint(file),
        RGB(0, 200, 155).paint(":"),
        RGB(0, 200, 155).paint(line),
        RGB(0, 200, 155).bold().paint(error)
    );
}
