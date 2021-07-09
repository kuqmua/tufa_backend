use ansi_term::Colour::RGB;

pub enum PrintType {
    Error,
    Warning,
    Success,
    PartialSuccess,
}

pub fn print_colorful_message(print_type: PrintType, file: String, line: String, message: String) {
    let rgb_color: ansi_term::Colour;
    match print_type {
        PrintType::Error => rgb_color = RGB(0, 255, 0),
        PrintType::Warning => rgb_color = RGB(0, 255, 0),
        PrintType::Success => rgb_color = RGB(0, 255, 0),
        PrintType::PartialSuccess => rgb_color = RGB(0, 255, 0),
    }
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
        rgb_color.paint("file: "),
        rgb_color.paint(file),
        rgb_color.paint(":"),
        rgb_color.paint(line),
        rgb_color.bold().paint(message)
    );
}
