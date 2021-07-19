use crate::check_net::check_link_metainfo_structures::HandledReachProviderStatusInfo;
use crate::check_net::check_link_metainfo_structures::UnhandledReachProviderInfo;
use crate::check_net::fetch_link::fetch_link;

use prints_lib::print_colorful_message;
use prints_lib::PrintType;

pub fn check_link(
    link: &str,
    enable_error_prints_handle: bool,
) -> (
    bool,
    UnhandledReachProviderInfo,
    HandledReachProviderStatusInfo,
) {
    let fetch_result = fetch_link(link);
    let mut can_i: bool = false;
    let unhandled_info: UnhandledReachProviderInfo;
    let handled_info: HandledReachProviderStatusInfo;
    match fetch_result {
        Ok(fetch_tuple_result) => {
            if fetch_tuple_result.0 {
                can_i = true;
                unhandled_info = UnhandledReachProviderInfo::Success;
                handled_info = fetch_tuple_result.1;
            } else {
                if enable_error_prints_handle {
                    match fetch_tuple_result.1 {
                        HandledReachProviderStatusInfo::ResStatusError(status_code) => {
                            let error_message = format!(
                                "{} ResStatusError: {}",
                                link,
                                status_code.clone().to_string()
                            );
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                file!().to_string(),
                                line!().to_string(),
                                error_message,
                            );
                        }
                        HandledReachProviderStatusInfo::Initialized => {
                            let error_message = format!("{} check_link false, and HandledReachProviderStatusInfo::Initialized??? wtf", link);
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                file!().to_string(),
                                line!().to_string(),
                                error_message,
                            );
                        }
                        HandledReachProviderStatusInfo::Success => {
                            let error_message = format!("{} check_link false, and HandledReachProviderStatusInfo::Success??? wtf", link);
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                file!().to_string(),
                                line!().to_string(),
                                error_message,
                            );
                        }
                    }
                }
                can_i = false;
                unhandled_info = UnhandledReachProviderInfo::Success;
                handled_info = fetch_tuple_result.1;
            }
        }
        Err(e) => {
            unhandled_info = UnhandledReachProviderInfo::Failure(e.to_string());
            handled_info = HandledReachProviderStatusInfo::Initialized;
            if enable_error_prints_handle {
                let error_message =
                    format!("{} check_link fetch_result Box<dyn Error> {}", link, e);
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    error_message,
                );
            }
        }
    }
    (can_i, unhandled_info, handled_info)
}
