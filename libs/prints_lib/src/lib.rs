use ansi_term::Colour::RGB;
use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
}

// // [enable_warning_prints]
// // enable_warning_prints_for_arxiv = true
// // enable_warning_prints_for_biorxiv = true
// // enable_warning_prints_for_github = true
// // enable_warning_prints_for_habr = true
// // enable_warning_prints_for_medrxiv = true
// // enable_warning_prints_for_reddit = true
// // enable_warning_prints_for_twitter = true

// // [enable_error_prints]
// // enable_error_prints_for_arxiv = true
// // enable_error_prints_for_biorxiv = true
// // enable_error_prints_for_github = true
// // enable_error_prints_for_habr = true
// // enable_error_prints_for_medrxiv = true
// // enable_error_prints_for_reddit = true
// // enable_error_prints_for_twitter = true

// // [enable_cleaning_warning_logs_directory]
// // enable_cleaning_warning_logs_directory_for_arxiv = true
// // enable_cleaning_warning_logs_directory_for_biorxiv = true
// // enable_cleaning_warning_logs_directory_for_github = true
// // enable_cleaning_warning_logs_directory_for_habr = true
// // enable_cleaning_warning_logs_directory_for_medrxiv = true
// // enable_cleaning_warning_logs_directory_for_reddit = true
// // enable_cleaning_warning_logs_directory_for_twitter = true

// // [enable_time_measurement]
// // enable_arxiv_time_measurement = true
// // enable_biorxiv_time_measurement = true
// // enable_github_time_measurement = true
// // enable_habr_time_measurement = true
// // enable_medrxiv_time_measurement = true
// // enable_reddit_time_measurement = true
// // enable_twitter_time_measurement = true
// pub fn print_colorful_message(
//     provider_kind: Option<ProviderKind>,
//     print_type: PrintType,
//     file: String,
//     line: String,
//     message: String,
// ) {
//     match provider_kind {
//         Some(provider_kind) => match provider_kind {
//             ProviderKind::Arxiv => {
//                 if CONFIG.enable_prints.enable_prints_arxiv {
//                     let rgb_color: ansi_term::Colour;
//                     match print_type {
//                         PrintType::Error => {
//                             rgb_color = RGB(
//                                 CONFIG.print_colors.error_red,
//                                 CONFIG.print_colors.error_green,
//                                 CONFIG.print_colors.error_blue,
//                             )
//                         }
//                         PrintType::WarningHigh => {
//                             rgb_color = RGB(
//                                 CONFIG.print_colors.warning_high_red,
//                                 CONFIG.print_colors.warning_high_green,
//                                 CONFIG.print_colors.warning_high_blue,
//                             )
//                         }
//                         PrintType::WarningLow => {
//                             rgb_color = RGB(
//                                 CONFIG.print_colors.warning_low_red,
//                                 CONFIG.print_colors.warning_low_green,
//                                 CONFIG.print_colors.warning_low_blue,
//                             )
//                         }
//                         PrintType::Success => {
//                             rgb_color = RGB(
//                                 CONFIG.print_colors.success_red,
//                                 CONFIG.print_colors.success_green,
//                                 CONFIG.print_colors.success_blue,
//                             )
//                         }
//                         PrintType::PartialSuccess => {
//                             rgb_color = RGB(
//                                 CONFIG.print_colors.partial_success_red,
//                                 CONFIG.print_colors.partial_success_green,
//                                 CONFIG.print_colors.partial_success_blue,
//                             )
//                         }
//                     }
//                 }
//             }
//             ProviderKind::Biorxiv => if CONFIG.enable_prints.enable_prints_biorxiv {},
//             ProviderKind::Github => if CONFIG.enable_prints.enable_prints_github {},
//             ProviderKind::Habr => if CONFIG.enable_prints.enable_prints_habr {},
//             ProviderKind::Medrxiv => if CONFIG.enable_prints.enable_prints_medrxiv {},
//             ProviderKind::Reddit => if CONFIG.enable_prints.enable_prints_reddit {},
//             ProviderKind::Twitter => if CONFIG.enable_prints.enable_prints_twitter {},
//         },
//         None => print_colorful_message_inner_part(
//             print_type: PrintType,
//             file: String,
//             line: String,
//             message: String,
//         ),
//     }
// }

// fn print_colorful_message_inner_part(
//     print_type: PrintType,
//     file: String,
//     line: String,
//     message: String,
// ) {
//     let rgb_color: ansi_term::Colour;
//     match print_type {
//         PrintType::Error => {
//             rgb_color = RGB(
//                 CONFIG.print_colors.error_red,
//                 CONFIG.print_colors.error_green,
//                 CONFIG.print_colors.error_blue,
//             )
//         }
//         PrintType::WarningHigh => {
//             rgb_color = RGB(
//                 CONFIG.print_colors.warning_high_red,
//                 CONFIG.print_colors.warning_high_green,
//                 CONFIG.print_colors.warning_high_blue,
//             )
//         }
//         PrintType::WarningLow => {
//             rgb_color = RGB(
//                 CONFIG.print_colors.warning_low_red,
//                 CONFIG.print_colors.warning_low_green,
//                 CONFIG.print_colors.warning_low_blue,
//             )
//         }
//         PrintType::Success => {
//             rgb_color = RGB(
//                 CONFIG.print_colors.success_red,
//                 CONFIG.print_colors.success_green,
//                 CONFIG.print_colors.success_blue,
//             )
//         }
//         PrintType::PartialSuccess => {
//             rgb_color = RGB(
//                 CONFIG.print_colors.partial_success_red,
//                 CONFIG.print_colors.partial_success_green,
//                 CONFIG.print_colors.partial_success_blue,
//             )
//         }
//     }
//     // debug!("something {}", error);
//     // error!("waaarn {}", error);
//     // println!("print error red, {}", error);
//     // let mut fileonos = File::create("errorlogs.txt").expect("could not create file");
//     // writeln!(&mut fileonos, "{}", error).unwrap();
//     // let result_of_writing = fileonos.write(error.as_bytes());
//     // match result_of_writing {
//     //     Ok(_) => println!("written"),
//     //     Err(e) => println!("error {}", e),
//     // }
//     eprintln!(
//         "{}{}{}{}\n{}",
//         rgb_color.paint("file: "),
//         rgb_color.paint(file),
//         rgb_color.paint(":"),
//         rgb_color.paint(line),
//         rgb_color.bold().paint(message)
//     );
// }

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
