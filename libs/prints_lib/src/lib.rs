use ansi_term::Colour::RGB;

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
pub enum PrintType {
    PrintColorRed { red: u8, green: u8, blue: u8 },
    PrintColorGreen { red: u8, green: u8, blue: u8 },
}

pub enum PrintColor {
    PrintColorRed { red: u8, green: u8, blue: u8 },
    PrintColorGreen { red: u8, green: u8, blue: u8 },
}
