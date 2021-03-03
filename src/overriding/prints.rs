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
    //     Ok(_) => println!("записано"),
    //     Err(e) => println!("error {}", e),
    // }
    eprintln!(
        "{}{}{}{}\n{}",
        Red.bold().paint("file: "),
        Red.bold().paint(file), //file.red().bold()
        Red.bold().paint(":"),
        Red.bold().paint(line),
        Red.bold().paint(error)
    );
}

pub fn print_warning_orange(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        RGB(255, 165, 0).bold().paint("file: "),
        RGB(255, 165, 0).bold().paint(file),
        RGB(255, 165, 0).bold().paint(":"),
        RGB(255, 165, 0).bold().paint(line),
        RGB(255, 165, 0).bold().paint(error)
    );
}

pub fn print_warning_yellow(file: String, line: String, error: String) {
    eprintln!(
        "{}{}{}{}\n{}",
        Yellow.bold().paint("file: "),
        Yellow.bold().paint(file),
        Yellow.bold().paint(":"),
        Yellow.bold().paint(line),
        Yellow.bold().paint(error)
    );
}
