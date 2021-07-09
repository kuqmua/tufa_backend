use ansi_term::Colour::RGB;
use config_lib::get_project_information::get_config::get_config_information::CONFIG;

pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
}

pub fn print_colorful_message(print_type: PrintType, file: String, line: String, message: String) {
    let rgb_color: ansi_term::Colour;
    match print_type {
        PrintType::Error => {
            rgb_color = RGB(
                CONFIG.print_colors.error_red,
                CONFIG.print_colors.error_green,
                CONFIG.print_colors.error_blue,
            )
        }
        PrintType::WarningHigh => {
            rgb_color = RGB(
                CONFIG.print_colors.warning_high_red,
                CONFIG.print_colors.warning_high_green,
                CONFIG.print_colors.warning_high_blue,
            )
        }
        PrintType::WarningLow => {
            rgb_color = RGB(
                CONFIG.print_colors.warning_low_red,
                CONFIG.print_colors.warning_low_green,
                CONFIG.print_colors.warning_low_blue,
            )
        }
        PrintType::Success => {
            rgb_color = RGB(
                CONFIG.print_colors.success_red,
                CONFIG.print_colors.success_green,
                CONFIG.print_colors.success_blue,
            )
        }
        PrintType::PartialSuccess => {
            rgb_color = RGB(
                CONFIG.print_colors.partial_success_red,
                CONFIG.print_colors.partial_success_green,
                CONFIG.print_colors.partial_success_blue,
            )
        }
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
