use ansi_term::Colour::RGB;
use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
    TimeMeasurement,
    CleaningWarningLogsDirectory,
}

// [enable_prints]
// enable_prints_arxiv = true -----------done
// enable_prints_biorxiv = true
// enable_prints_github = true
// enable_prints_habr = true
// enable_prints_medrxiv = true
// enable_prints_reddit = true
// enable_prints_twitter = true

// [enable_warning_high_prints]
// enable_warning_high_prints_for_arxiv = true -----------done
// enable_warning_high_prints_for_biorxiv = true
// enable_warning_high_prints_for_github = true
// enable_warning_high_prints_for_habr = true
// enable_warning_high_prints_for_medrxiv = true
// enable_warning_high_prints_for_reddit = true
// enable_warning_high_prints_for_twitter = true

// [enable_warning_low_prints]
// enable_warning_low_prints_for_arxiv = true -----------done
// enable_warning_low_prints_for_biorxiv = true
// enable_warning_low_prints_for_github = true
// enable_warning_low_prints_for_habr = true
// enable_warning_low_prints_for_medrxiv = true
// enable_warning_low_prints_for_reddit = true
// enable_warning_low_prints_for_twitter = true

// [enable_error_prints]
// enable_error_prints_for_arxiv = true -----------done
// enable_error_prints_for_biorxiv = true
// enable_error_prints_for_github = true
// enable_error_prints_for_habr = true
// enable_error_prints_for_medrxiv = true
// enable_error_prints_for_reddit = true
// enable_error_prints_for_twitter = true

// [enable_success_prints]
// enable_success_prints_for_arxiv = true -----------done
// enable_success_prints_for_biorxiv = true
// enable_success_prints_for_github = true
// enable_success_prints_for_habr = true
// enable_success_prints_for_medrxiv = true
// enable_success_prints_for_reddit = true
// enable_success_prints_for_twitter = true

// [enable_partial_success_prints]
// enable_partial_success_prints_for_arxiv = true -----------done
// enable_partial_success_prints_for_biorxiv = true
// enable_partial_success_prints_for_github = true
// enable_partial_success_prints_for_habr = true
// enable_partial_success_prints_for_medrxiv = true
// enable_partial_success_prints_for_reddit = true
// enable_partial_success_prints_for_twitter = true

// [enable_cleaning_warning_logs_directory]
// enable_cleaning_warning_logs_directory_for_arxiv = true -----------done
// enable_cleaning_warning_logs_directory_for_biorxiv = true
// enable_cleaning_warning_logs_directory_for_github = true
// enable_cleaning_warning_logs_directory_for_habr = true
// enable_cleaning_warning_logs_directory_for_medrxiv = true
// enable_cleaning_warning_logs_directory_for_reddit = true
// enable_cleaning_warning_logs_directory_for_twitter = true

// [enable_time_measurement]
// enable_arxiv_time_measurement = true -----------done
// enable_biorxiv_time_measurement = true
// enable_github_time_measurement = true
// enable_habr_time_measurement = true
// enable_medrxiv_time_measurement = true
// enable_reddit_time_measurement = true
// enable_twitter_time_measurement = true

// [params]
// enable_warning_high_prints_for_all_providers = true
// enable_warning_low_prints_for_all_providers = true
// enable_error_prints_for_all_providers = true
// enable_all_cleaning_warning_logs_directory = true
// enable_prints_handle = true
// enable_error_prints_handle = true

// pub fn print_colorful_message(
//     provider_kind: Option<ProviderKind>,
//     print_type: PrintType,
//     file: String,
//     line: String,
//     message: String,
// ) {
//     match provider_kind {
//         Some(provider_kind) => {
//             if CONFIG.params.enable_all_providers_prints {
//                 match provider_kind {
//                     ProviderKind::Arxiv => {
//                         if CONFIG.enable_prints.enable_prints_arxiv {
//                             match print_type {
//                                 PrintType::Error => {
//                                     if CONFIG.enable_error_prints.enable_error_prints_for_arxiv {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.error_red,
//                                             CONFIG.print_colors.error_green,
//                                             CONFIG.print_colors.error_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                                 PrintType::WarningHigh => {
//                                     if CONFIG
//                                         .enable_warning_high_prints
//                                         .enable_warning_high_prints_for_arxiv
//                                     {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.warning_high_red,
//                                             CONFIG.print_colors.warning_high_green,
//                                             CONFIG.print_colors.warning_high_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                                 PrintType::WarningLow => {
//                                     if CONFIG
//                                         .enable_warning_low_prints
//                                         .enable_warning_low_prints_for_arxiv
//                                     {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.warning_low_red,
//                                             CONFIG.print_colors.warning_low_green,
//                                             CONFIG.print_colors.warning_low_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                                 PrintType::Success => {
//                                     if CONFIG.enable_success_prints.enable_success_prints_for_arxiv
//                                     {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.success_red,
//                                             CONFIG.print_colors.success_green,
//                                             CONFIG.print_colors.success_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                                 PrintType::PartialSuccess => {
//                                     if CONFIG
//                                         .enable_partial_success_prints
//                                         .enable_partial_success_prints_for_arxiv
//                                     {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.partial_success_red,
//                                             CONFIG.print_colors.partial_success_green,
//                                             CONFIG.print_colors.partial_success_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                                 PrintType::TimeMeasurement => {
//                                     if CONFIG.enable_time_measurement.enable_arxiv_time_measurement
//                                     {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.time_measurement_red,
//                                             CONFIG.print_colors.time_measurement_green,
//                                             CONFIG.print_colors.time_measurement_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                                 PrintType::CleaningWarningLogsDirectory => {
//                                     if CONFIG
//                                         .enable_cleaning_warning_logs_directory
//                                         .enable_cleaning_warning_logs_directory_for_arxiv
//                                     {
//                                         let rgb_color: ansi_term::Colour = RGB(
//                                             CONFIG.print_colors.cleaning_red,
//                                             CONFIG.print_colors.cleaning_green,
//                                             CONFIG.print_colors.cleaning_blue,
//                                         );
//                                         eprintln!(
//                                             "{}{}{}{}\n{}",
//                                             rgb_color.paint("file: "),
//                                             rgb_color.paint(file),
//                                             rgb_color.paint(":"),
//                                             rgb_color.paint(line),
//                                             rgb_color.bold().paint(message)
//                                         );
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     ProviderKind::Biorxiv => if CONFIG.enable_prints.enable_prints_biorxiv {},
//                     ProviderKind::Github => if CONFIG.enable_prints.enable_prints_github {},
//                     ProviderKind::Habr => if CONFIG.enable_prints.enable_prints_habr {},
//                     ProviderKind::Medrxiv => if CONFIG.enable_prints.enable_prints_medrxiv {},
//                     ProviderKind::Reddit => if CONFIG.enable_prints.enable_prints_reddit {},
//                     ProviderKind::Twitter => if CONFIG.enable_prints.enable_prints_twitter {},
//                 }
//             }
//         }
//         None => match print_type {
//             PrintType::Error => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.error_red,
//                     CONFIG.print_colors.error_green,
//                     CONFIG.print_colors.error_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//             PrintType::WarningHigh => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.warning_high_red,
//                     CONFIG.print_colors.warning_high_green,
//                     CONFIG.print_colors.warning_high_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//             PrintType::WarningLow => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.warning_low_red,
//                     CONFIG.print_colors.warning_low_green,
//                     CONFIG.print_colors.warning_low_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//             PrintType::Success => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.success_red,
//                     CONFIG.print_colors.success_green,
//                     CONFIG.print_colors.success_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//             PrintType::PartialSuccess => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.partial_success_red,
//                     CONFIG.print_colors.partial_success_green,
//                     CONFIG.print_colors.partial_success_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//             PrintType::TimeMeasurement => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.time_measurement_red,
//                     CONFIG.print_colors.time_measurement_green,
//                     CONFIG.print_colors.time_measurement_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//             PrintType::CleaningWarningLogsDirectory => {
//                 let rgb_color: ansi_term::Colour = RGB(
//                     CONFIG.print_colors.cleaning_red,
//                     CONFIG.print_colors.cleaning_green,
//                     CONFIG.print_colors.cleaning_blue,
//                 );
//                 eprintln!(
//                     "{}{}{}{}\n{}",
//                     rgb_color.paint("file: "),
//                     rgb_color.paint(file),
//                     rgb_color.paint(":"),
//                     rgb_color.paint(line),
//                     rgb_color.bold().paint(message)
//                 );
//             }
//         },
//     }
// }

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

//////
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
        PrintType::TimeMeasurement => {
            rgb_color = RGB(
                CONFIG.print_colors.time_measurement_red,
                CONFIG.print_colors.time_measurement_green,
                CONFIG.print_colors.time_measurement_blue,
            )
        }
        PrintType::CleaningWarningLogsDirectory => {
            rgb_color = RGB(
                CONFIG.print_colors.cleaning_red,
                CONFIG.print_colors.cleaning_green,
                CONFIG.print_colors.cleaning_blue,
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
